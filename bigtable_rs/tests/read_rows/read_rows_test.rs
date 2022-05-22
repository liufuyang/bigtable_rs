use bigtable_rs::bigtable::read_rows::decode_read_rows_response_to_vec;
use bigtable_rs::google::bigtable::v2::read_rows_response::cell_chunk::RowStatus;
use bigtable_rs::google::bigtable::v2::read_rows_response::CellChunk;
use bigtable_rs::google::cloud::conformace::bigtable::v2::TestFile;
use std::fs;

/// `read_row_test.json` is a straight copy from Google's
/// https://github.com/googleapis/conformance-tests/blob/master/bigtable/v2/readrows.json
/// And this test is basically a clone version of
/// https://github.com/googleapis/java-bigtable/blob/main/google-cloud-bigtable/src/test/java/com/google/cloud/bigtable/data/v2/stub/readrows/ReadRowsMergingAcceptanceTest.java

#[test]
fn tmp_test_1() {
    let chunk = CellChunk {
        row_key: vec![],
        family_name: None,
        qualifier: None,
        timestamp_micros: 0,
        labels: vec![],
        value: vec![],
        value_size: 0,
        row_status: Some(RowStatus::CommitRow(true)),
    };

    println!("{}", serde_json::to_string_pretty(&chunk).unwrap());
}

#[test]
fn it_adds_two() {
    let file = fs::File::open("tests/read_rows/read_rows_test.simple.json")
        .expect("file should open read only");

    let jd = &mut serde_json::Deserializer::from_reader(file);
    let result: Result<TestFile, _> = serde_path_to_error::deserialize(jd);
    let test_file = match result {
        Ok(test_file) => test_file,
        Err(err) => {
            panic!("Json parse error: {}", err);
        }
    };

    for (test_id, t) in test_file.read_rows_tests.into_iter().enumerate() {
        let decode_read_rows_result = decode_read_rows_response_to_vec(t.chunks);
        println!("{} Testing: {}", test_id, t.description);

        for (i, expected) in t.results.into_iter().enumerate() {
            if expected.error {
                assert!(decode_read_rows_result.get(i).unwrap().is_err());
            } else {
                let (key, _cells) = decode_read_rows_result
                    .as_slice()
                    .get(i)
                    .unwrap()
                    .as_ref()
                    .unwrap();
                assert_eq!(String::from_utf8_lossy(key.as_slice()), expected.row_key);
                // assert_eq!("", expected.value);
            }
        }
    }

    assert_eq!(4, 4);
}
