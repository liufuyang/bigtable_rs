#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestFile {
    #[prost(message, repeated, tag = "1")]
    pub read_rows_tests: ::prost::alloc::vec::Vec<ReadRowsTest>,
}
#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsTest {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub chunks: ::prost::alloc::vec::Vec<
        super::super::super::super::bigtable::v2::read_rows_response::CellChunk,
    >,
    #[prost(message, repeated, tag = "3")]
    pub results: ::prost::alloc::vec::Vec<read_rows_test::Result>,
}
/// Nested message and enum types in `ReadRowsTest`.
pub mod read_rows_test {
    /// Expected results of reading the row.
    /// Only the last result can be an error.
    #[serde_with::serde_as]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(default)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(string, tag = "1")]
        pub row_key: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub family_name: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub qualifier: ::prost::alloc::string::String,
        #[prost(int64, tag = "4")]
        #[serde_as(as = "serde_with::DisplayFromStr")]
        pub timestamp_micros: i64,
        #[prost(string, tag = "5")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub label: ::prost::alloc::string::String,
        #[prost(bool, tag = "7")]
        pub error: bool,
    }
}
