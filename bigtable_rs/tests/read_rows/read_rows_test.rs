use bigtable_rs::bigtable::read_rows::decode_read_rows_response;

/// `read_row_test.json` is a straight copy from Google's
/// https://github.com/googleapis/conformance-tests/blob/master/bigtable/v2/readrows.json

#[test]
fn it_adds_two() {
    assert_eq!(4, 4);
}
