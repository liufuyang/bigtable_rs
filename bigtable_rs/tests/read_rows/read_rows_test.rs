use std::fs;

use crate::read_rows::google::cloud::conformace::bigtable::v2::{
    read_rows_test, ReadRowsTest, TestFile,
};
use crate::read_rows::types_with_serde::CellChunkWithSerde;
use bigtable_rs::bigtable::read_rows::decode_read_rows_response_to_vec;
use bigtable_rs::google::bigtable::v2::read_rows_response::CellChunk;

type ReadRowTestResult = read_rows_test::Result;

/// `read_row_test.json` is a straight copy from Google's
/// https://github.com/googleapis/conformance-tests/blob/master/bigtable/v2/readrows.json
/// And this test is basically a clone version of
/// https://github.com/googleapis/java-bigtable/blob/main/google-cloud-bigtable/src/test/java/com/google/cloud/bigtable/data/v2/stub/readrows/ReadRowsMergingAcceptanceTest.java

#[test]
fn tmp_test_1() {
    let chunk = CellChunkWithSerde {
        row_key: vec![],
        family_name: None,
        qualifier: None,
        timestamp_micros: 0,
        labels: vec![],
        value: vec![],
        value_size: 0,
        commit_row: Some(true),
        reset_row: Some(false),
    };

    println!("{}", serde_json::to_string_pretty(&chunk).unwrap());
}

#[test]
fn read_rows_test_from_json_can_all_pass() {
    let file =
        fs::File::open("tests/read_rows/read_rows_test.json").expect("file should open read only");

    let jd = &mut serde_json::Deserializer::from_reader(file);
    let result: Result<TestFile, _> = serde_path_to_error::deserialize(jd);
    let test_file = match result {
        Ok(test_file) => test_file,
        Err(err) => {
            panic!("Json parse error: {}", err);
        }
    };

    for (test_id, mut rrt) in test_file.read_rows_tests.into_iter().enumerate() {
        let chunks: Vec<CellChunk> = rrt.chunks.into_iter().map(CellChunk::from).collect();
        let decode_read_rows_results = decode_read_rows_response_to_vec(chunks);
        rrt.chunks = vec![]; // fill in partial moved chunks, let borrow checker pass

        println!("{} Testing: {}", test_id, rrt.description);

        let actual_result_has_error = decode_read_rows_results.iter().any(|ar| {
            println!("{:?}", ar);
            ar.is_err()
        });
        let expect_result_has_error = expect_read_row_test_result_has_error(&rrt);

        if expect_result_has_error != actual_result_has_error {
            panic!(
                "Expect having error: {} not match with actual having error: {}",
                expect_result_has_error, actual_result_has_error
            );
        }

        let expected_results: Vec<ReadRowTestResult> =
            rrt.results.into_iter().filter(|r| !r.error).collect();
        let actual_results: Vec<ReadRowTestResult> = decode_read_rows_results
            .into_iter()
            .filter(|r| r.is_ok())
            .flat_map(|r| {
                let (row_key, row_cells) = r.unwrap();
                row_cells.into_iter().map(move |cell| ReadRowTestResult {
                    row_key: String::from_utf8(row_key.clone()).unwrap(),
                    family_name: cell.family_name,
                    qualifier: String::from_utf8(cell.qualifier).unwrap(),
                    timestamp_micros: cell.timestamp_micros,
                    value: String::from_utf8(cell.value).unwrap(),
                    label: cell.labels.first().unwrap_or(&"".to_owned()).clone(),
                    error: false,
                })
            })
            .collect();
        assert_eq!(expected_results, actual_results);
    }
}

fn expect_read_row_test_result_has_error(rrt: &ReadRowsTest) -> bool {
    rrt.results.last().map(|r| r.error).unwrap_or(false)
}
