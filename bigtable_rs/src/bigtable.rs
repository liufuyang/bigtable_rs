//! `bigtable` module provides a few convenient structs for calling Google Bigtable from Rust code.
//!
//!
//! Example usage:
//! ```rust,no_run
//! use bigtable_rs::bigtable;
//! use bigtable_rs::google::bigtable::v2::row_filter::{Chain, Filter};
//! use bigtable_rs::google::bigtable::v2::row_range::{EndKey, StartKey};
//! use bigtable_rs::google::bigtable::v2::{ReadRowsRequest, RowFilter, RowRange, RowSet};
//! use env_logger;
//! use std::error::Error;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     env_logger::init();
//!
//!     let project_id = "project-id";
//!     let instance_name = "instance-1";
//!     let table_name = "table-1";
//!     let channel_size = 4;
//!     let timeout = Duration::from_secs(10);
//!
//!     let key_start: String = "key1".to_owned();
//!     let key_end: String = "key4".to_owned();
//!
//!     // make a bigtable client
//!     let connection = bigtable::BigTableConnection::new(
//!         project_id,
//!         instance_name,
//!         true,
//!         channel_size,
//!         Some(timeout),
//!     )
//!         .await?;
//!     let mut bigtable = connection.client();
//!
//!     // prepare a ReadRowsRequest
//!     let request = ReadRowsRequest {
//!         app_profile_id: "default".to_owned(),
//!         table_name: bigtable.get_full_table_name(table_name),
//!         rows_limit: 10,
//!         rows: Some(RowSet {
//!             row_keys: vec![], // use this field to put keys for reading specific rows
//!             row_ranges: vec![RowRange {
//!                 start_key: Some(StartKey::StartKeyClosed(key_start.into_bytes())),
//!                 end_key: Some(EndKey::EndKeyOpen(key_end.into_bytes())),
//!             }],
//!         }),
//!         filter: Some(RowFilter {
//!             filter: Some(Filter::Chain(Chain {
//!                 filters: vec![
//!                     RowFilter {
//!                         filter: Some(Filter::FamilyNameRegexFilter("cf1".to_owned())),
//!                     },
//!                     RowFilter {
//!                         filter: Some(Filter::ColumnQualifierRegexFilter("c1".as_bytes().to_vec())),
//!                     },
//!                     RowFilter {
//!                         filter: Some(Filter::CellsPerColumnLimitFilter(1)),
//!                     },
//!                 ],
//!             })),
//!         }),
//!         ..ReadRowsRequest::default()
//!     };
//!
//!     // calling bigtable API to get results
//!     let response = bigtable.read_rows(request).await?;
//!
//!     // simply print results for example usage
//!     response.into_iter().for_each(|(key, data)| {
//!         println!("------------\n{}", String::from_utf8(key.clone()).unwrap());
//!         data.into_iter().for_each(|row_cell| {
//!             println!(
//!                 "    [{}:{}] \"{}\" @ {}",
//!                 row_cell.family_name,
//!                 String::from_utf8(row_cell.qualifier).unwrap(),
//!                 String::from_utf8(row_cell.value).unwrap(),
//!                 row_cell.timestamp_micros
//!             )
//!         })
//!     });
//!
//!     Ok(())
//! }
//! ```

use crate::google::bigtable::v2::{
    bigtable_client::BigtableClient, read_rows_response::cell_chunk::RowStatus, MutateRowRequest,
    MutateRowResponse, MutateRowsRequest, MutateRowsResponse, ReadRowsRequest, ReadRowsResponse,
    RowRange, RowSet, SampleRowKeysRequest, SampleRowKeysResponse,
};

use crate::google::bigtable::v2::row_range::{EndKey, StartKey};
use crate::{
    access_token::{AccessToken, Scope},
    root_ca_certificate,
    util::get_end_key,
};
use log::{info, trace, warn};
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tonic::service::interceptor::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::Endpoint;
use tonic::{
    codec::Streaming, metadata::MetadataValue, transport::Channel, transport::ClientTlsConfig,
    Response, Status,
};

/// An alias for Vec<u8> as row key
type RowKey = Vec<u8>;
/// A convenient Result type
type Result<T> = std::result::Result<T, Error>;
type BigtableClientIntercepted = BigtableClient<InterceptedService<Channel, BTInterceptor>>;

/// A data structure for returning the read content of a cell in a row.
pub struct RowCell {
    pub family_name: String,
    pub qualifier: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp_micros: i64,
}

/// Error types the client may have
#[derive(Debug, Error)]
pub enum Error {
    #[error("AccessToken error: {0}")]
    AccessTokenError(String),

    #[error("Certificate error: {0}")]
    CertificateError(String),

    #[error("I/O Error: {0}")]
    IoError(std::io::Error),

    #[error("Transport error: {0}")]
    TransportError(tonic::transport::Error),

    #[error("Row not found")]
    RowNotFound,

    #[error("Row write failed")]
    RowWriteFailed,

    #[error("Object not found: {0}")]
    ObjectNotFound(String),

    #[error("Object is corrupt: {0}")]
    ObjectCorrupt(String),

    #[error("RPC error: {0}")]
    RpcError(tonic::Status),

    #[error("Timeout error after {0} seconds")]
    TimeoutError(u64),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl std::convert::From<tonic::transport::Error> for Error {
    fn from(err: tonic::transport::Error) -> Self {
        Self::TransportError(err)
    }
}

impl std::convert::From<tonic::Status> for Error {
    fn from(err: tonic::Status) -> Self {
        Self::RpcError(err)
    }
}

/// For initiate a Bigtable connection, then a `Bigtable` client can be made from it.
#[derive(Clone)]
pub struct BigTableConnection {
    access_token: Arc<Option<AccessToken>>,
    channel: tonic::transport::Channel,
    table_prefix: Arc<String>,
    timeout: Arc<Option<Duration>>,
}

impl BigTableConnection {
    /// Establish a connection to the BigTable instance named `instance_name`.  If read-only access
    /// is required, the `read_only` flag should be used to reduce the requested OAuth2 scope.
    ///
    /// The GOOGLE_APPLICATION_CREDENTIALS environment variable will be used to determine the
    /// program name that contains the BigTable instance in addition to access credentials.
    ///
    /// The BIGTABLE_EMULATOR_HOST environment variable is also respected.
    ///
    /// `channel_size` defines the number of connections (or channels) established to Bigtable
    /// service, and the requests are load balanced onto all the channels. You must therefore
    /// make sure all of these connections are open when a new request is to be sent.
    /// Idle connections are automatically closed in "a few minutes". Therefore it is important to
    /// make sure you have a high enough QPS to send at least one request through all the
    /// connections (in every service host) every minute. If not, you should consider decreasing the
    /// channel size. If you are not sure what value to pick and your load is low, just start with 1.
    /// The recommended value could be 2 x the thread count in your tokio environment see info here
    /// https://docs.rs/tokio/latest/tokio/attr.main.html, but it might be a very different case for
    /// different applications.
    ///
    pub async fn new(
        project_id: &str,
        instance_name: &str,
        read_only: bool,
        channel_size: usize,
        timeout: Option<Duration>,
    ) -> Result<Self> {
        match std::env::var("BIGTABLE_EMULATOR_HOST") {
            Ok(endpoint) => {
                info!("Connecting to bigtable emulator at {}", endpoint);
                let endpoints: Vec<Endpoint> = vec![0; channel_size.max(1)]
                    .iter()
                    .map(move |_| {
                        Channel::from_shared(format!("http://{}", endpoint))
                            .expect("Invalid connection emulator uri")
                            .http2_keep_alive_interval(Duration::from_secs(60))
                            .keep_alive_while_idle(true)
                    })
                    .map(|ep| {
                        if let Some(timeout) = timeout {
                            ep.timeout(timeout)
                        } else {
                            ep
                        }
                    })
                    .collect();

                Ok(Self {
                    access_token: Arc::new(None),
                    channel: Channel::balance_list(endpoints.into_iter()),
                    table_prefix: Arc::new(format!(
                        "projects/emulator/instances/{}/tables/",
                        instance_name
                    )),
                    timeout: Arc::new(timeout),
                })
            }

            Err(_) => {
                let access_token = AccessToken::new(if read_only {
                    Scope::BigTableDataReadOnly
                } else {
                    Scope::BigTableData
                })
                    .await
                    .map_err(Error::AccessTokenError)?;

                let table_prefix = format!(
                    "projects/{}/instances/{}/tables/",
                    project_id, instance_name
                );

                let endpoints: Result<Vec<Endpoint>> = vec![0; channel_size.max(1)]
                    .iter()
                    .map(move |_| {
                        Channel::from_static("https://bigtable.googleapis.com")
                            .tls_config(
                                ClientTlsConfig::new()
                                    .ca_certificate(
                                        root_ca_certificate::load()
                                            .map_err(Error::CertificateError)
                                            .expect("root certificate error"),
                                    )
                                    .domain_name("bigtable.googleapis.com"),
                            )
                            .map_err(Error::TransportError)
                    })
                    .collect();

                let endpoints: Vec<Endpoint> = endpoints?
                    .into_iter()
                    .map(|ep| {
                        ep.http2_keep_alive_interval(Duration::from_secs(60))
                            .keep_alive_while_idle(true)
                    })
                    .map(|ep| {
                        if let Some(timeout) = timeout {
                            ep.timeout(timeout)
                        } else {
                            ep
                        }
                    })
                    .collect();

                Ok(Self {
                    access_token: Arc::new(Some(access_token)),
                    channel: Channel::balance_list(endpoints.into_iter()),
                    table_prefix: Arc::new(table_prefix),
                    timeout: Arc::new(timeout),
                })
            }
        }
    }

    /// Create a new BigTable client.
    ///
    /// Clients require `&mut self`, due to `Tonic::transport::Channel` limitations, however
    /// the created new clients can be cheaply cloned and thus can be send to different threads
    pub fn client(&self) -> BigTable {
        let client = BigtableClient::with_interceptor(
            self.channel.clone(),
            BTInterceptor {
                access_token: self.access_token.as_ref().clone(),
            },
        );

        BigTable {
            access_token: self.access_token.clone(),
            client,
            table_prefix: self.table_prefix.clone(),
            timeout: self.timeout.clone(),
        }
    }
}

/// The core struct for Bigtable client, witch wraps a gPRC client defined by Bigtable proto.
/// In order easy share this struct in multiple thread, we only store references here, besides the
/// `BigtableClient` as it wraps a tonic Channel and cloning on is cheap.
#[derive(Clone)]
pub struct BigTable {
    access_token: Arc<Option<AccessToken>>,
    client: BigtableClientIntercepted,
    // clone is cheap with Channel, see https://docs.rs/tonic/latest/tonic/transport/struct.Channel.html
    table_prefix: Arc<String>,
    timeout: Arc<Option<Duration>>,
}

impl BigTable {
    /// Wrapped `read_rows` method
    pub async fn read_rows(
        &mut self,
        request: ReadRowsRequest,
    ) -> Result<Vec<(RowKey, Vec<RowCell>)>> {
        self.refresh_access_token().await;
        let response = self.client.read_rows(request).await?.into_inner();
        self.decode_read_rows_response(response).await
    }

    /// Provide `read_rows_with_prefix` method to allow using a prefix as key
    pub async fn read_rows_with_prefix(
        &mut self,
        mut request: ReadRowsRequest,
        prefix: Vec<u8>,
    ) -> Result<Vec<(RowKey, Vec<RowCell>)>> {
        self.refresh_access_token().await;
        let end_key = get_end_key(prefix.as_ref()).map(|end_key| EndKey::EndKeyOpen(end_key));
        request.rows = Some(RowSet {
            row_keys: vec![], // use this field to put keys for reading specific rows
            row_ranges: vec![RowRange {
                start_key: Some(StartKey::StartKeyClosed(prefix)),
                end_key,
            }],
        });
        let response = self.client.read_rows(request).await?.into_inner();
        self.decode_read_rows_response(response).await
    }

    /// Wrapped `sample_row_keys` method
    pub async fn sample_row_keys(
        &mut self,
        request: SampleRowKeysRequest,
    ) -> Result<Streaming<SampleRowKeysResponse>> {
        self.refresh_access_token().await;
        let response = self.client.sample_row_keys(request).await?.into_inner();
        Ok(response)
    }

    /// Wrapped `mutate_row` method
    pub async fn mutate_row(
        &mut self,
        request: MutateRowRequest,
    ) -> Result<Response<MutateRowResponse>> {
        self.refresh_access_token().await;
        let response = self.client.mutate_row(request).await?;
        Ok(response)
    }

    /// Wrapped `mutate_rows` method
    pub async fn mutate_rows(
        &mut self,
        request: MutateRowsRequest,
    ) -> Result<Streaming<MutateRowsResponse>> {
        self.refresh_access_token().await;
        let response = self.client.mutate_rows(request).await?.into_inner();
        Ok(response)
    }

    /// Provide a convenient method to get the inner `BigtableClient` so user can use any methods
    /// defined from the Bigtable V2 gRPC API
    pub fn get_client(&mut self) -> &mut BigtableClientIntercepted {
        &mut self.client
    }

    /// Only needed to call this if you interact with the internal `BigtableClient` directly.
    /// Call this method to ensure the access token refreshed.
    pub async fn refresh_access_token(&self) {
        if let Some(ref access_token) = self.access_token.as_ref() {
            access_token.refresh().await;
        }
    }

    /// Provide a convenient method to get full table, which can be used for building requests
    pub fn get_full_table_name(&self, table_name: &str) -> String {
        [&self.table_prefix, table_name].concat()
    }

    /// As each `CellChunk` could be only part of a cell, this method reorganize multiple `CellChunk`
    /// from multiple `ReadRowsResponse` into a `Vec<(RowKey, Vec<RowCell>)>`.
    async fn decode_read_rows_response(
        &self,
        mut rrr: Streaming<ReadRowsResponse>,
    ) -> Result<Vec<(RowKey, Vec<RowCell>)>> {
        let mut rows: Vec<(RowKey, Vec<RowCell>)> = vec![];

        let mut row_key = None;
        let mut row_data: Vec<RowCell> = vec![];

        let mut cell_family_name = None;
        let mut cell_name = None;
        let mut cell_timestamp = 0;
        let mut cell_value = vec![];

        let started = Instant::now();

        while let Some(res) = rrr.message().await? {
            if let Some(timeout) = self.timeout.as_ref() {
                if Instant::now().duration_since(started) > *timeout {
                    return Err(Error::TimeoutError(timeout.as_secs()));
                }
            }
            for (i, mut chunk) in res.chunks.into_iter().enumerate() {
                // The comments for `read_rows_response::CellChunk` provide essential details for
                // understanding how the below decoding works...
                trace!("chunk {}: {:?}", i, chunk.value);

                // Starting a new row?
                if !chunk.row_key.is_empty() {
                    row_key = Some(chunk.row_key);
                }

                // Starting a new cell? A new cell will have a qualifier and a family
                if let Some(chunk_qualifier) = chunk.qualifier {
                    // New cell begins. Check whether previous cell_name exist, if so then it means
                    // the cell_value is not empty and previous cell is not closed up. So close up the previous cell.
                    if let Some(cell_name) = cell_name {
                        let row_cell = RowCell {
                            family_name: cell_family_name.take().unwrap_or("".to_owned()),
                            qualifier: cell_name,
                            value: cell_value,
                            timestamp_micros: cell_timestamp,
                        };
                        row_data.push(row_cell);
                        cell_value = vec![];
                    }
                    cell_name = Some(chunk_qualifier);
                    cell_family_name = chunk.family_name;
                    cell_timestamp = chunk.timestamp_micros;
                }
                cell_value.append(&mut chunk.value);

                // End of a row?
                match chunk.row_status {
                    None => {
                        // more for this row, don't push to row_data or rows vector, let the next
                        // chunk close up those vectors.
                    }
                    Some(RowStatus::CommitRow(_)) => {
                        // End of a row, closing up the cell, then close this row
                        if let Some(cell_name) = cell_name.take() {
                            let row_cell = RowCell {
                                family_name: cell_family_name.take().unwrap_or("".to_owned()),
                                qualifier: cell_name,
                                value: cell_value,
                                timestamp_micros: cell_timestamp,
                            };
                            row_data.push(row_cell);
                            cell_value = vec![];
                        } else {
                            warn!("Row ended with cell_name=None. This should not happen.")
                        }

                        if let Some(row_key) = row_key.take() {
                            rows.push((row_key, row_data));
                            row_data = vec![];
                        }
                    }
                    Some(RowStatus::ResetRow(_)) => {
                        // ResetRow indicates that the client should drop all previous chunks for
                        // `row_key`, as it will be re-read from the beginning.
                        row_key = None;
                        row_data = vec![];
                    }
                }
            }
        }
        Ok(rows)
    }
}

#[derive(Clone)]
pub struct BTInterceptor {
    access_token: Option<AccessToken>,
}

impl Interceptor for BTInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, Status> {
        if let Some(token) = self.access_token.as_ref() {
            match MetadataValue::from_str(&token.get()) {
                Ok(authorization_header) => {
                    request
                        .metadata_mut()
                        .insert("authorization", authorization_header);
                }
                Err(err) => {
                    warn!("Failed to set authorization header: {}", err);
                }
            }
            Ok(request)
        } else {
            Ok(request)
        }
    }
}
