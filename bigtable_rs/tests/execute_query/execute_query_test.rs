use std::fs;

use bigtable_rs::bigtable::execute_query::merge_partial_results;
use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::{
    partial_result_set, value, PartialResultSet, ProtoRows, ProtoRowsBatch, Value,
};
use prost::Message as _;

use crate::execute_query::types::{ExecuteQueryTestFile, TestMessage};

/// Build a `PartialResultSet` from a JSON test message.
///
/// `raw_values` are encoded as `ProtoRows { values: [RawValue(...)] }` and
/// appended to `accumulated_buf` (which mirrors the server's perspective of the
/// buffer up to this point).  When `with_checksum` is set the CRC is computed
/// over the full accumulated buffer and included in the message; the buffer
/// position is then reset.  When `bad_checksum` is set a deliberate wrong
/// checksum (0xDEAD_BEEF) is used instead.
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

/// Extract the raw bytes from a `Value` that carries a `RawValue` kind.
/// Panics with a helpful message if the kind differs.
fn raw_bytes(v: &Value) -> Vec<u8> {
    match &v.kind {
        Some(value::Kind::RawValue(b)) => b.clone(),
        other => panic!("expected RawValue, got {other:?}"),
    }
}

#[test]
fn execute_query_merging_test_from_json() {
    let file = fs::File::open("tests/execute_query/execute_query_test.json")
        .expect("test json file should exist");

    let jd = &mut serde_json::Deserializer::from_reader(file);
    let result: Result<ExecuteQueryTestFile, _> = serde_path_to_error::deserialize(jd);
    let test_file = match result {
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

        let actual = merge_partial_results(messages, test.num_columns);

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
                .map(|row| row.iter().map(raw_bytes).collect())
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
