use bigtable_rs::google::bigtable::v2::read_rows_response::cell_chunk::RowStatus;
use bigtable_rs::google::bigtable::v2::read_rows_response::CellChunk;

use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;

use prost::Message;

/// A proxy type to [`bigtable_rs::google::bigtable::v2::read_rows_response::CellChunk`],
/// with serde tags applied, so that we can use test data from json file.
#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[derive(Clone, PartialEq, Message)]
pub struct CellChunkWithSerde {
    #[serde_as(as = "Base64")]
    #[serde(default)]
    #[prost(bytes = "vec", tag = "1")]
    pub row_key: Vec<u8>,

    #[prost(message, optional, tag = "2")]
    pub family_name: Option<String>,

    // Note: Prost uses Option<Vec<u8>> for some optional bytes
    #[serde_as(as = "Option<Base64>")]
    #[serde(default)]
    #[prost(bytes = "vec", optional, tag = "3")]
    pub qualifier: Option<Vec<u8>>,

    #[serde(default)]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[prost(int64, tag = "4")]
    pub timestamp_micros: i64,

    #[serde(default)]
    #[prost(string, repeated, tag = "5")]
    pub labels: Vec<String>,

    #[serde_as(as = "Base64")]
    #[prost(bytes = "vec", tag = "6")]
    pub value: Vec<u8>,

    #[prost(int32, tag = "7")]
    pub value_size: i32,

    // Handled as a separate step or simplified if you don't need OneOf logic right now
    #[serde(default)]
    #[prost(bool, optional, tag = "8")]
    pub commit_row: Option<bool>,

    #[serde(default)]
    #[prost(bool, optional, tag = "9")]
    pub reset_row: Option<bool>,
}

impl From<CellChunkWithSerde> for CellChunk {
    fn from(proxy: CellChunkWithSerde) -> Self {
        // Map the flat proxy fields back to the Protobuf OneOf enum
        let row_status = if let Some(true) = proxy.commit_row {
            Some(RowStatus::CommitRow(true))
        } else if let Some(true) = proxy.reset_row {
            Some(RowStatus::ResetRow(true))
        } else {
            None
        };

        Self {
            row_key: proxy.row_key,
            family_name: proxy.family_name,
            qualifier: proxy.qualifier,
            timestamp_micros: proxy.timestamp_micros,
            labels: proxy.labels,
            value: proxy.value,
            value_size: proxy.value_size,
            row_status,
        }
    }
}
