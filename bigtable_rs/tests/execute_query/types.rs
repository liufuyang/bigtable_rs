use serde::Deserialize;
use serde_with::base64::Base64;
use serde_with::serde_as;

/// Top-level container for the JSON test file.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQueryTestFile {
    pub execute_query_tests: Vec<ExecuteQueryTest>,
}

/// One test case: a description, column count, a sequence of messages to feed
/// through the merge algorithm, and the expected non-None yields.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ExecuteQueryTest {
    pub description: String,
    pub num_columns: usize,
    pub messages: Vec<TestMessage>,
    pub results: Vec<TestResult>,
}

impl Default for ExecuteQueryTest {
    fn default() -> Self {
        Self {
            description: String::new(),
            num_columns: 1,
            messages: Vec::new(),
            results: Vec::new(),
        }
    }
}

/// One PartialResultSet-equivalent for the test.
///
/// Instead of raw proto `batch_data` bytes, logical values are listed so the
/// JSON stays human-readable.  The test harness encodes `raw_values` into a
/// serialized `ProtoRows` and computes the CRC over the accumulated buffer.
#[serde_as]
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct TestMessage {
    /// Raw byte values (base64) to place in the batch.  Empty = no ProtoRows.
    #[serde_as(as = "Vec<Base64>")]
    pub raw_values: Vec<Vec<u8>>,
    /// Include the correct CRC32C of the accumulated buffer in this message.
    pub with_checksum: bool,
    /// Include a deliberately wrong checksum (0xDEADBEEF) to trigger a CRC error.
    pub bad_checksum: bool,
    /// Base64-encoded resume token bytes.
    #[serde_as(as = "Base64")]
    pub resume_token: Vec<u8>,
    /// Set the reset flag on this message.
    pub reset: bool,
}

/// Expected output from one non-None process_partial yield.
#[serde_as]
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct TestResult {
    /// If true, an error is expected; `rows` and `token` are ignored.
    pub error: bool,
    /// Expected decoded rows.  Each row is a list of base64-encoded raw values.
    #[serde_as(as = "Vec<Vec<Base64>>")]
    pub rows: Vec<Vec<Vec<u8>>>,
    /// Expected resume token (base64), or absent/null for no token.
    #[serde_as(as = "Option<Base64>")]
    pub token: Option<Vec<u8>>,
}
