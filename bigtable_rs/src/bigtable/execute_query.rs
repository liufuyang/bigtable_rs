use futures_util::Stream;
use prost::Message as _;

use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::{
    execute_query_response, partial_result_set, ExecuteQueryResponse, PartialResultSet, ProtoRows,
    Value,
};

use crate::bigtable::{Error, Result};

// Accumulates ProtoRowsBatch.batch_data fragments and tracks the column count.
// Kept separate from StreamState so tests can construct it without a live gRPC
// stream.
struct MergeState {
    buffer: Vec<u8>,
    num_columns: usize,
}

struct StreamState {
    current_stream: tonic::Streaming<ExecuteQueryResponse>,
    merge: MergeState,
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
    p: googleapis_tonic_google_bigtable_v2::google::bigtable::v2::PartialResultSet,
    state: &mut MergeState,
) -> Result<Option<(Vec<Vec<Value>>, Option<Vec<u8>>)>> {
    if p.reset {
        state.buffer.clear();
    }

    // Dispatch on format.  Currently only ProtoFormat exists in the proto.
    // When ArrowFormat is added, extend this match with a new arm.
    match p.partial_rows {
        Some(partial_result_set::PartialRows::ProtoRowsBatch(batch)) => {
            state.buffer.extend_from_slice(&batch.batch_data);
        }
        None => {}
    }

    let mut rows = Vec::new();

    if let Some(expected_crc) = p.batch_checksum {
        // Always compute CRC regardless of whether buffer is empty.  If an
        // unknown format variant arrives (prost calls skip_field on it, leaving
        // the buffer empty), the server's CRC will be over non-empty data and
        // will not match our CRC of an empty buffer, surfacing an error instead
        // of silently dropping the data.
        let actual_crc = crc32c::crc32c(&state.buffer);
        if actual_crc != expected_crc {
            return Err(Error::InvalidValue(format!(
                "CRC32C mismatch: expected {expected_crc:#010x}, got {actual_crc:#010x}"
            )));
        }

        if !state.buffer.is_empty() {
            let proto_rows = ProtoRows::decode(state.buffer.as_slice())
                .map_err(|e| Error::InvalidValue(format!("ProtoRows decode error: {e}")))?;
            state.buffer.clear();

            if !proto_rows.values.is_empty() {
                if state.num_columns == 0 {
                    return Err(Error::InvalidValue(
                        "server sent row values but ResultSetMetadata has zero columns".to_string(),
                    ));
                }
                if proto_rows.values.len() % state.num_columns != 0 {
                    return Err(Error::InvalidValue(format!(
                        "server sent {} values but schema has {} columns (not divisible)",
                        proto_rows.values.len(),
                        state.num_columns
                    )));
                }
                rows = proto_rows
                    .values
                    .chunks(state.num_columns)
                    .map(|row| row.to_vec())
                    .collect();
            }
        }
    }

    let token = if !p.resume_token.is_empty() {
        // A non-empty buffer here means the server emitted a resume_token while
        // a batch was still open — batch_data arrived without a preceding
        // batch_checksum.  This is a protocol violation.
        if !state.buffer.is_empty() {
            return Err(Error::InvalidValue(
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

/// Build a stream of `(rows, resume_token)` items from an already-started
/// ExecuteQuery RPC stream.
///
/// Each item is `(Vec<Vec<Value>>, Option<Vec<u8>>)`:
/// - `rows` is yielded as soon as a `batch_checksum` verifies the batch —
///   callers receive rows without waiting for the next `resume_token`.
/// - `resume_token` is `Some` only at checkpoint boundaries.  Pass it in a
///   subsequent `ExecuteQueryRequest.resume_token` to resume the query from
///   that checkpoint without re-reading earlier results.
/// - Both may be present in the same item when a single message carries a
///   `batch_checksum` and a `resume_token` together.
///
/// Values in each row are in column order per
/// `ResultSetMetadata.proto_schema.columns`.  Errors surface as stream items
/// and terminate the stream.
pub fn make_stream(
    stream: tonic::Streaming<ExecuteQueryResponse>,
    num_columns: usize,
) -> impl Stream<Item = Result<(Vec<Vec<Value>>, Option<Vec<u8>>)>> {
    let state = StreamState {
        current_stream: stream,
        merge: MergeState {
            buffer: Vec::new(),
            num_columns,
        },
    };

    // The unfold state is Option<StreamState>: None signals the stream is done.
    futures_util::stream::unfold(Some(state), |state_opt| async move {
        let mut state = state_opt?;

        loop {
            match state.current_stream.message().await {
                // gRPC stream closed cleanly.  If the buffer is non-empty the
                // server closed mid-batch without a final batch_checksum —
                // surface an error rather than silently dropping the data.
                Ok(None) => {
                    if !state.merge.buffer.is_empty() {
                        return Some((
                            Err(Error::InvalidValue(
                                "stream closed with unverified data in batch buffer (missing batch_checksum)"
                                    .to_string(),
                            )),
                            None,
                        ));
                    }
                    return None;
                }

                Ok(Some(msg)) => match msg.response {
                    // Metadata is only sent on the initial connection; skip it
                    // if it unexpectedly appears mid-stream.
                    Some(execute_query_response::Response::Metadata(_)) => continue,

                    Some(execute_query_response::Response::Results(p)) => {
                        match process_partial(p, &mut state.merge) {
                            Err(e) => return Some((Err(e), None)),
                            Ok(None) => continue,
                            Ok(Some((rows, token))) => {
                                return Some((Ok((rows, token)), Some(state)))
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

/// Run the PartialResultSet merge algorithm over a sequence of pre-fetched
/// messages, returning each non-empty yield as a `Result` item.
///
/// Useful for testing the merge logic without a live gRPC stream; production
/// code should use [`make_stream`] instead.
pub fn merge_partial_results(
    messages: impl IntoIterator<Item = PartialResultSet>,
    num_columns: usize,
) -> Vec<Result<(Vec<Vec<Value>>, Option<Vec<u8>>)>> {
    let mut state = MergeState {
        buffer: Vec::new(),
        num_columns,
    };
    let mut out = Vec::new();
    for msg in messages {
        match process_partial(msg, &mut state) {
            Err(e) => {
                out.push(Err(e));
                return out;
            }
            Ok(None) => {}
            Ok(Some(item)) => out.push(Ok(item)),
        }
    }
    out
}
