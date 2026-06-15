use futures_util::Stream;
use prost::Message as _;

use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::{
    execute_query_response, partial_result_set, ExecuteQueryResponse, PartialResultSet, ProtoRows,
};

use crate::bigtable::{Error, ExecuteQueryRetryContext, Result, SqlRow};

// Accumulates ProtoRowsBatch.batch_data fragments and tracks the column count.
// Kept separate from StreamState so tests can construct it without a live gRPC
// stream.
struct MergeState {
    buffer: Vec<u8>,
    num_columns: usize,
}

// Process one PartialResultSet message.
//
// Implements the merge algorithm:
//   reset        → discard buffer
//   data         → append to buffer
//   checksum     → verify CRC32C, decode buffer as ProtoRows, yield rows immediately
//   resume_token → attach checkpoint token to the item
//
// Returns Ok(Some((rows, token))) when there is something to surface:
//   rows non-empty, token None  — verified batch ready, no checkpoint yet
//   rows empty,    token Some   — progress-only checkpoint
//   rows non-empty, token Some  — batch + checkpoint in the same message
// Returns Ok(None) when more messages are needed.
// Returns Err on checksum/decode failure or protocol violation.
fn process_partial(
    p: PartialResultSet,
    state: &mut MergeState,
) -> Result<Option<(Vec<SqlRow>, Option<Vec<u8>>)>> {
    if p.reset {
        state.buffer.clear();
    }

    // Dispatch on format.  Currently only ProtoFormat exists in the proto.
    // When ArrowFormat is added, extend this match with a new arm.
    if let Some(partial_result_set::PartialRows::ProtoRowsBatch(batch)) = p.partial_rows {
        state.buffer.extend_from_slice(&batch.batch_data);
    }

    let mut rows: Vec<SqlRow> = Vec::new();

    if let Some(expected_crc) = p.batch_checksum {
        // Always compute CRC regardless of whether buffer is empty.  If an
        // unknown format variant arrives (prost calls skip_field on it, leaving
        // the buffer empty), the server's CRC will be over non-empty data and
        // will not match our CRC of an empty buffer, surfacing an error instead
        // of silently dropping the data.
        let actual_crc = crc32c::crc32c(&state.buffer);
        if actual_crc != expected_crc {
            return Err(Error::ChecksumMismatch {
                expected: expected_crc,
                actual: actual_crc,
            });
        }

        if !state.buffer.is_empty() {
            let proto_rows = ProtoRows::decode(state.buffer.as_slice())
                .map_err(|e| Error::ProtocolViolation(format!("ProtoRows decode error: {e}")))?;
            state.buffer.clear();

            if !proto_rows.values.is_empty() {
                if state.num_columns == 0 {
                    return Err(Error::ProtocolViolation(
                        "server sent row values but ResultSetMetadata has zero columns".to_string(),
                    ));
                }
                if proto_rows.values.len() % state.num_columns != 0 {
                    return Err(Error::ProtocolViolation(format!(
                        "server sent {} values but schema has {} columns (not divisible)",
                        proto_rows.values.len(),
                        state.num_columns
                    )));
                }
                let n_rows = proto_rows.values.len() / state.num_columns;
                let mut values = proto_rows.values.into_iter();
                rows = (0..n_rows)
                    .map(|_| SqlRow(values.by_ref().take(state.num_columns).collect()))
                    .collect();
            }
        }
    }

    let token = if !p.resume_token.is_empty() {
        // A non-empty buffer here means the server emitted a resume_token while
        // a batch was still open — batch_data arrived without a preceding
        // batch_checksum.  This is a protocol violation.
        if !state.buffer.is_empty() {
            return Err(Error::ProtocolViolation(
                "resume_token received while batch buffer is non-empty (missing batch_checksum)"
                    .to_string(),
            ));
        }
        Some(p.resume_token)
    } else {
        None
    };

    if rows.is_empty() && token.is_none() {
        return Ok(None);
    }

    Ok(Some((rows, token)))
}

/// Build a stream of decoded row batches from an already-started ExecuteQuery
/// RPC stream.
///
/// Each item is `Result<(Vec<SqlRow>, Option<ExecuteQueryRetryContext>)>`:
/// - `rows` is yielded as soon as a `batch_checksum` verifies the batch —
///   callers receive rows without waiting for the next checkpoint.
/// - `retry_context` is `Some` only at checkpoint boundaries.  Pass it back
///   to `execute_query_stream` as `retry_context: Some(ctx)` to resume from
///   that point without re-reading earlier results.
/// - Both may be present in the same item when a message carries a checksum
///   and a resume token together.
///
/// Values in each row are in column order per
/// `ResultSetMetadata.proto_schema.columns`.  Errors surface as stream items
/// and terminate the stream.
pub(crate) fn make_stream(
    stream: tonic::Streaming<ExecuteQueryResponse>,
    num_columns: usize,
    prepared_query: Vec<u8>,
) -> impl Stream<Item = Result<(Vec<SqlRow>, Option<ExecuteQueryRetryContext>)>> {
    struct State {
        current_stream: tonic::Streaming<ExecuteQueryResponse>,
        merge: MergeState,
        prepared_query: Vec<u8>,
    }

    let state = State {
        current_stream: stream,
        merge: MergeState {
            buffer: Vec::new(),
            num_columns,
        },
        prepared_query,
    };

    futures_util::stream::unfold(Some(state), |state_opt| async move {
        let mut state = state_opt?;

        loop {
            match state.current_stream.message().await {
                Ok(None) => {
                    if !state.merge.buffer.is_empty() {
                        return Some((
                            Err(Error::ProtocolViolation(
                                "stream closed with unverified data in batch buffer (missing batch_checksum)"
                                    .to_string(),
                            )),
                            None,
                        ));
                    }
                    return None;
                }

                Ok(Some(msg)) => match msg.response {
                    Some(execute_query_response::Response::Metadata(_)) => continue,

                    Some(execute_query_response::Response::Results(p)) => {
                        match process_partial(p, &mut state.merge) {
                            Err(e) => return Some((Err(e), None)),
                            Ok(None) => continue,
                            Ok(Some((rows, raw_token))) => {
                                let retry_context =
                                    raw_token.map(|token| ExecuteQueryRetryContext {
                                        prepared_query: state.prepared_query.clone(),
                                        resume_token: token,
                                    });
                                return Some((Ok((rows, retry_context)), Some(state)));
                            }
                        }
                    }

                    None => continue,
                },

                Err(status) => return Some((Err(Error::RpcError(status)), None)),
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{process_partial, MergeState};
    use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::{
        partial_result_set, value, PartialResultSet, ProtoRows, ProtoRowsBatch, Value,
    };
    use prost::Message as _;
    use serde::Deserialize;
    use serde_with::base64::Base64;
    use serde_with::serde_as;
    use std::fs;

    // ── JSON test-file types ──────────────────────────────────────────────────

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ExecuteQueryTestFile {
        execute_query_tests: Vec<ExecuteQueryTest>,
    }

    #[derive(Deserialize, Default)]
    #[serde(rename_all = "camelCase", default)]
    struct ExecuteQueryTest {
        description: String,
        num_columns: usize,
        messages: Vec<TestMessage>,
        results: Vec<TestResult>,
    }

    #[serde_as]
    #[derive(Deserialize, Default)]
    #[serde(rename_all = "camelCase", default)]
    struct TestMessage {
        #[serde_as(as = "Vec<Base64>")]
        raw_values: Vec<Vec<u8>>,
        with_checksum: bool,
        bad_checksum: bool,
        #[serde_as(as = "Base64")]
        resume_token: Vec<u8>,
        reset: bool,
    }

    #[serde_as]
    #[derive(Deserialize, Default)]
    #[serde(rename_all = "camelCase", default)]
    struct TestResult {
        error: bool,
        #[serde_as(as = "Vec<Vec<Base64>>")]
        rows: Vec<Vec<Vec<u8>>>,
        #[serde_as(as = "Option<Base64>")]
        token: Option<Vec<u8>>,
    }

    // ── Test helpers ──────────────────────────────────────────────────────────

    fn build_message(tm: TestMessage, accumulated_buf: &mut Vec<u8>) -> PartialResultSet {
        if tm.reset {
            accumulated_buf.clear();
        }
        let batch_data: Option<Vec<u8>> = if !tm.raw_values.is_empty() {
            let values: Vec<Value> = tm
                .raw_values
                .into_iter()
                .map(|b| Value {
                    kind: Some(value::Kind::RawValue(b)),
                    r#type: None,
                })
                .collect();
            let encoded = ProtoRows { values }.encode_to_vec();
            accumulated_buf.extend_from_slice(&encoded);
            Some(encoded)
        } else {
            None
        };
        let batch_checksum = if tm.with_checksum {
            let crc = crc32c::crc32c(accumulated_buf);
            accumulated_buf.clear();
            Some(crc)
        } else if tm.bad_checksum {
            Some(0xDEAD_BEEF_u32)
        } else {
            None
        };
        PartialResultSet {
            partial_rows: batch_data.map(|d| {
                partial_result_set::PartialRows::ProtoRowsBatch(ProtoRowsBatch { batch_data: d })
            }),
            batch_checksum,
            resume_token: tm.resume_token,
            reset: tm.reset,
            estimated_batch_size: 0,
        }
    }

    fn raw_bytes(v: &Value) -> Vec<u8> {
        match &v.kind {
            Some(value::Kind::RawValue(b)) => b.clone(),
            other => panic!("expected RawValue, got {other:?}"),
        }
    }

    // ── Tests ─────────────────────────────────────────────────────────────────

    /// `execute_query_test.json` was hand-written to cover the PartialResultSet merge
    /// algorithm described in the Bigtable ExecuteQuery protocol documentation.
    #[test]
    fn execute_query_merging_test_from_json() {
        let file = fs::File::open("tests/execute_query/execute_query_test.json")
            .expect("test json file should exist");
        let jd = &mut serde_json::Deserializer::from_reader(file);
        let test_file: ExecuteQueryTestFile = match serde_path_to_error::deserialize(jd) {
            Ok(f) => f,
            Err(e) => panic!("JSON parse error: {e}"),
        };

        for (i, test) in test_file.execute_query_tests.into_iter().enumerate() {
            println!("{i} Testing: {}", test.description);

            let mut buf: Vec<u8> = Vec::new();
            let messages: Vec<PartialResultSet> = test
                .messages
                .into_iter()
                .map(|tm| build_message(tm, &mut buf))
                .collect();

            let mut state = MergeState {
                buffer: Vec::new(),
                num_columns: test.num_columns,
            };
            let mut actual = Vec::new();
            for msg in messages {
                match process_partial(msg, &mut state) {
                    Err(e) => {
                        actual.push(Err(e));
                        break;
                    }
                    Ok(None) => {}
                    Ok(Some(item)) => actual.push(Ok(item)),
                }
            }

            let actual_has_error = actual.iter().any(|r| r.is_err());
            let expect_error = test.results.last().map(|r| r.error).unwrap_or(false);
            assert_eq!(
                expect_error, actual_has_error,
                "test {i} '{}': error expectation mismatch",
                test.description
            );

            let expected_ok: Vec<_> = test.results.iter().filter(|r| !r.error).collect();
            let actual_ok: Vec<_> = actual.into_iter().filter_map(|r| r.ok()).collect();
            assert_eq!(
                expected_ok.len(),
                actual_ok.len(),
                "test {i} '{}': result count mismatch",
                test.description
            );

            for (j, (exp, act)) in expected_ok.iter().zip(actual_ok.iter()).enumerate() {
                let actual_rows: Vec<Vec<Vec<u8>>> = act
                    .0
                    .iter()
                    .map(|row| row.0.iter().map(raw_bytes).collect())
                    .collect();
                assert_eq!(
                    exp.rows, actual_rows,
                    "test {i} '{}' result {j}: rows mismatch",
                    test.description
                );
                assert_eq!(
                    exp.token, act.1,
                    "test {i} '{}' result {j}: token mismatch",
                    test.description
                );
            }
        }
    }
}
