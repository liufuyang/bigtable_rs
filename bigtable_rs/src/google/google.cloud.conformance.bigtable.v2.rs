// @generated
// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestFile {
    #[prost(message, repeated, tag="1")]
    pub read_rows_tests: ::prost::alloc::vec::Vec<ReadRowsTest>,
}
impl ::prost::Name for TestFile {
const NAME: &'static str = "TestFile";
const PACKAGE: &'static str = "google.cloud.conformance.bigtable.v2";
fn full_name() -> ::prost::alloc::string::String { "google.cloud.conformance.bigtable.v2.TestFile".into() }fn type_url() -> ::prost::alloc::string::String { "/google.cloud.conformance.bigtable.v2.TestFile".into() }}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsTest {
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub chunks: ::prost::alloc::vec::Vec<super::super::super::super::bigtable::v2::read_rows_response::CellChunk>,
    #[prost(message, repeated, tag="3")]
    pub results: ::prost::alloc::vec::Vec<read_rows_test::Result>,
}
/// Nested message and enum types in `ReadRowsTest`.
pub mod read_rows_test {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(string, tag="1")]
        pub row_key: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub family_name: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub qualifier: ::prost::alloc::string::String,
        #[prost(int64, tag="4")]
        pub timestamp_micros: i64,
        #[prost(string, tag="5")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag="6")]
        pub label: ::prost::alloc::string::String,
        #[prost(bool, tag="7")]
        pub error: bool,
    }
impl ::prost::Name for Result {
const NAME: &'static str = "Result";
const PACKAGE: &'static str = "google.cloud.conformance.bigtable.v2";
fn full_name() -> ::prost::alloc::string::String { "google.cloud.conformance.bigtable.v2.ReadRowsTest.Result".into() }fn type_url() -> ::prost::alloc::string::String { "/google.cloud.conformance.bigtable.v2.ReadRowsTest.Result".into() }}
}
impl ::prost::Name for ReadRowsTest {
const NAME: &'static str = "ReadRowsTest";
const PACKAGE: &'static str = "google.cloud.conformance.bigtable.v2";
fn full_name() -> ::prost::alloc::string::String { "google.cloud.conformance.bigtable.v2.ReadRowsTest".into() }fn type_url() -> ::prost::alloc::string::String { "/google.cloud.conformance.bigtable.v2.ReadRowsTest".into() }}
/// Encoded file descriptor set for the `google.cloud.conformance.bigtable.v2` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x92, 0x12, 0x0a, 0x18, 0x74, 0x65, 0x73, 0x74, 0x2f, 0x62, 0x69, 0x67, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x5f, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x24, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6c, 0x6f, 0x75, 0x64, 0x2e, 0x63, 0x6f, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x62, 0x69, 0x67, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x2e, 0x76, 0x32, 0x1a, 0x21, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x62, 0x69, 0x67, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x2f, 0x76, 0x32, 0x2f, 0x62, 0x69, 0x67, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x66, 0x0a, 0x08, 0x54, 0x65, 0x73, 0x74, 0x46, 0x69,
    0x6c, 0x65, 0x12, 0x5a, 0x0a, 0x0f, 0x72, 0x65, 0x61, 0x64, 0x5f, 0x72, 0x6f, 0x77, 0x73, 0x5f,
    0x74, 0x65, 0x73, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6c, 0x6f, 0x75, 0x64, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x62, 0x69, 0x67, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e,
    0x76, 0x32, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x52, 0x6f, 0x77, 0x73, 0x54, 0x65, 0x73, 0x74, 0x52,
    0x0d, 0x72, 0x65, 0x61, 0x64, 0x52, 0x6f, 0x77, 0x73, 0x54, 0x65, 0x73, 0x74, 0x73, 0x22, 0x9d,
    0x03, 0x0a, 0x0c, 0x52, 0x65, 0x61, 0x64, 0x52, 0x6f, 0x77, 0x73, 0x54, 0x65, 0x73, 0x74, 0x12,
    0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x46, 0x0a, 0x06, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x2e, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x62, 0x69, 0x67, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x2e, 0x76, 0x32, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x52, 0x6f, 0x77, 0x73, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x43, 0x65, 0x6c, 0x6c, 0x43, 0x68, 0x75, 0x6e,
    0x6b, 0x52, 0x06, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x73, 0x12, 0x53, 0x0a, 0x07, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x39, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6c, 0x6f, 0x75, 0x64, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x6e, 0x63, 0x65, 0x2e, 0x62, 0x69, 0x67, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x76,
    0x32, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x52, 0x6f, 0x77, 0x73, 0x54, 0x65, 0x73, 0x74, 0x2e, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x07, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x1a, 0xcd,
    0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x72, 0x6f, 0x77,
    0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x72, 0x6f, 0x77, 0x4b,
    0x65, 0x79, 0x12, 0x1f, 0x0a, 0x0b, 0x66, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x66, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x4e,
    0x61, 0x6d, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x12, 0x29, 0x0a, 0x10, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x6d,
    0x69, 0x63, 0x72, 0x6f, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0f, 0x74, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x4d, 0x69, 0x63, 0x72, 0x6f, 0x73, 0x12, 0x14, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x05, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x42, 0x8d,
    0x01, 0x0a, 0x28, 0x63, 0x6f, 0x6d, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6c,
    0x6f, 0x75, 0x64, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65, 0x2e,
    0x62, 0x69, 0x67, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x76, 0x32, 0x42, 0x0e, 0x54, 0x65, 0x73,
    0x74, 0x44, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5a, 0x24, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x63, 0x6c, 0x6f, 0x75, 0x64, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x6e, 0x63, 0x65, 0x2f, 0x62, 0x69, 0x67, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2f, 0x76,
    0x32, 0xaa, 0x02, 0x2a, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x43, 0x6c, 0x6f, 0x75, 0x64,
    0x2e, 0x42, 0x69, 0x67, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x56, 0x32, 0x2e, 0x54, 0x65, 0x73,
    0x74, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65, 0x4a, 0x8c,
    0x0c, 0x0a, 0x06, 0x12, 0x04, 0x0e, 0x00, 0x2f, 0x01, 0x0a, 0xbe, 0x04, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x0e, 0x00, 0x12, 0x32, 0xb3, 0x04, 0x20, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68,
    0x74, 0x20, 0x32, 0x30, 0x31, 0x39, 0x2c, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x4c,
    0x4c, 0x43, 0x0a, 0x0a, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x64, 0x20, 0x75, 0x6e,
    0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x4c,
    0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2c, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x32, 0x2e, 0x30, 0x20, 0x28, 0x74, 0x68, 0x65, 0x20, 0x22, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73,
    0x65, 0x22, 0x29, 0x3b, 0x0a, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f,
    0x74, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20,
    0x65, 0x78, 0x63, 0x65, 0x70, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x69,
    0x61, 0x6e, 0x63, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69,
    0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20,
    0x6f, 0x62, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x74, 0x0a,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x77, 0x77,
    0x77, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x6c, 0x69, 0x63,
    0x65, 0x6e, 0x73, 0x65, 0x73, 0x2f, 0x4c, 0x49, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x2d, 0x32, 0x2e,
    0x30, 0x0a, 0x0a, 0x20, 0x55, 0x6e, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69,
    0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x6c, 0x61, 0x77, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x67, 0x72, 0x65, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x72, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x73,
    0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c,
    0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x22, 0x41, 0x53, 0x20,
    0x49, 0x53, 0x22, 0x20, 0x42, 0x41, 0x53, 0x49, 0x53, 0x2c, 0x0a, 0x20, 0x57, 0x49, 0x54, 0x48,
    0x4f, 0x55, 0x54, 0x20, 0x57, 0x41, 0x52, 0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53, 0x20, 0x4f,
    0x52, 0x20, 0x43, 0x4f, 0x4e, 0x44, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x53, 0x20, 0x4f, 0x46, 0x20,
    0x41, 0x4e, 0x59, 0x20, 0x4b, 0x49, 0x4e, 0x44, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72,
    0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x70, 0x6c,
    0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69,
    0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70,
    0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x20,
    0x67, 0x6f, 0x76, 0x65, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x10, 0x00, 0x2d, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x12, 0x00, 0x2b, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x14, 0x00, 0x47, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x25, 0x12,
    0x03, 0x14, 0x00, 0x47, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x15, 0x00, 0x2f, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x15, 0x00, 0x2f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x16, 0x00, 0x41, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x16, 0x00, 0x41, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x17, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03,
    0x17, 0x00, 0x3b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x19, 0x00, 0x1b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x1a, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a,
    0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x2a, 0x2b,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1d, 0x00, 0x2f, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x14, 0x0a, 0x5b, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00,
    0x12, 0x04, 0x21, 0x02, 0x29, 0x03, 0x1a, 0x4d, 0x20, 0x45, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65,
    0x64, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x61,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x77, 0x2e, 0x0a, 0x20, 0x4f,
    0x6e, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03,
    0x21, 0x0a, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x22,
    0x04, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22,
    0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22,
    0x0b, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22,
    0x15, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x23, 0x04,
    0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x23, 0x04,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x0b,
    0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x19,
    0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x24, 0x04, 0x19,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x24, 0x04, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x0b, 0x14,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x24, 0x17, 0x18,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x25, 0x04, 0x1f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x25, 0x04, 0x09, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x25, 0x0a, 0x1a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x25, 0x1d, 0x1e, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x04, 0x12, 0x03, 0x26, 0x04, 0x15, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x26, 0x04, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x26, 0x0b, 0x10, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x26, 0x13, 0x14, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05, 0x12, 0x03, 0x27, 0x04, 0x15, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x27, 0x04, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x27, 0x0b, 0x10, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x27, 0x13, 0x14, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x12, 0x03, 0x28, 0x04, 0x13, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x28, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x28, 0x09, 0x0e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x28, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x2b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2b, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x2b, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x44,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2c, 0x0b, 0x38, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x39, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x2c, 0x42, 0x43, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x2d, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2d, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x12, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x1c, 0x1d, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("google.cloud.conformance.bigtable.v2.serde.rs");
// @@protoc_insertion_point(module)