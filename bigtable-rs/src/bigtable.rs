use crate::google::bigtable::v2::{
    bigtable_client::BigtableClient, read_rows_response::cell_chunk::RowStatus, row_filter,
    ReadRowsRequest, ReadRowsResponse, RowFilter, RowSet,
};

use crate::{
    access_token::{AccessToken, Scope},
    root_ca_certificate,
};
use log::{info, trace, warn};
use std::time::{Duration, Instant};
use thiserror::Error;
use tonic::{metadata::MetadataValue, transport::ClientTlsConfig, Request};

pub type RowKey = String;
pub type RowData = Vec<(CellName, CellValue)>;
pub type RowDataSlice<'a> = &'a [(CellName, CellValue)];
pub type CellName = String;
pub type CellValue = Vec<u8>;

pub enum CellData<B, P> {
    Bincode(B),
    Protobuf(P),
}

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

    #[error("Invalid URI {0}: {1}")]
    InvalidUri(String, String),

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

    #[error("Timeout error")]
    TimeoutError,
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

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct BigTableConnection {
    access_token: Option<AccessToken>,
    channel: tonic::transport::Channel,
    table_prefix: String,
    timeout: Option<Duration>,
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
    pub async fn new(
        instance_name: &str,
        read_only: bool,
        timeout: Option<Duration>,
    ) -> Result<Self> {
        match std::env::var("BIGTABLE_EMULATOR_HOST") {
            Ok(endpoint) => {
                info!("Connecting to bigtable emulator at {}", endpoint);

                Ok(Self {
                    access_token: None,
                    channel: tonic::transport::Channel::from_shared(format!("http://{}", endpoint))
                        .map_err(|err| Error::InvalidUri(endpoint, err.to_string()))?
                        .connect_lazy()?,
                    table_prefix: format!("projects/emulator/instances/{}/tables/", instance_name),
                    timeout,
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
                    access_token.project(),
                    instance_name
                );

                let endpoint = {
                    let endpoint =
                        tonic::transport::Channel::from_static("https://bigtable.googleapis.com")
                            .tls_config(
                            ClientTlsConfig::new()
                                .ca_certificate(
                                    root_ca_certificate::load().map_err(Error::CertificateError)?,
                                )
                                .domain_name("bigtable.googleapis.com"),
                        )?;

                    if let Some(timeout) = timeout {
                        endpoint.timeout(timeout)
                    } else {
                        endpoint
                    }
                };

                Ok(Self {
                    access_token: Some(access_token),
                    channel: endpoint.connect_lazy()?,
                    table_prefix,
                    timeout,
                })
            }
        }
    }

    /// Create a new BigTable client.
    ///
    /// Clients require `&mut self`, due to `Tonic::transport::Channel` limitations, however
    /// creating new clients is cheap and thus can be used as a work around for ease of use.
    pub fn client(&self) -> BigTable {
        let client = if let Some(access_token) = &self.access_token {
            let access_token = access_token.clone();
            BigtableClient::with_interceptor(self.channel.clone(), move |mut req: Request<()>| {
                match MetadataValue::from_str(&access_token.get()) {
                    Ok(authorization_header) => {
                        req.metadata_mut()
                            .insert("authorization", authorization_header);
                    }
                    Err(err) => {
                        warn!("Failed to set authorization header: {}", err);
                    }
                }
                Ok(req)
            })
        } else {
            BigtableClient::new(self.channel.clone())
        };
        BigTable {
            access_token: self.access_token.clone(),
            client,
            table_prefix: self.table_prefix.clone(),
            timeout: self.timeout,
        }
    }
}

pub struct BigTable {
    access_token: Option<AccessToken>,
    client: BigtableClient<tonic::transport::Channel>,
    table_prefix: String,
    timeout: Option<Duration>,
}

impl BigTable {
    /// Get latest data from a single row of `table`, if that row exists. Returns an error if that
    /// row does not exist.
    ///
    /// All column families are accepted, and only the latest version of each column cell will be
    /// returned.
    pub async fn get_single_row_data(
        &mut self,
        table_name: &str,
        row_key: RowKey,
    ) -> Result<RowData> {
        self.refresh_access_token().await;

        let response = self
            .client
            .read_rows(ReadRowsRequest {
                table_name: format!("{}{}", self.table_prefix, table_name),
                rows_limit: 1,
                rows: Some(RowSet {
                    row_keys: vec![row_key.into_bytes()],
                    row_ranges: vec![],
                }),
                filter: Some(RowFilter {
                    // Only return the latest version of each cell
                    filter: Some(row_filter::Filter::CellsPerColumnLimitFilter(1)),
                }),
                ..ReadRowsRequest::default()
            })
            .await?
            .into_inner();

        let rows = self.decode_read_rows_response(response).await?;
        rows.into_iter()
            .next()
            .map(|r| r.1)
            .ok_or(Error::RowNotFound)
    }

    async fn refresh_access_token(&self) {
        if let Some(ref access_token) = self.access_token {
            access_token.refresh().await;
        }
    }

    async fn decode_read_rows_response(
        &self,
        mut rrr: tonic::codec::Streaming<ReadRowsResponse>,
    ) -> Result<Vec<(RowKey, RowData)>> {
        let mut rows: Vec<(RowKey, RowData)> = vec![];

        let mut row_key = None;
        let mut row_data = vec![];

        let mut cell_name = None;
        let mut cell_timestamp = 0;
        let mut cell_value = vec![];
        let mut cell_version_ok = true;
        let started = Instant::now();

        while let Some(res) = rrr.message().await? {
            if let Some(timeout) = self.timeout {
                if Instant::now().duration_since(started) > timeout {
                    return Err(Error::TimeoutError);
                }
            }
            for (i, mut chunk) in res.chunks.into_iter().enumerate() {
                // The comments for `read_rows_response::CellChunk` provide essential details for
                // understanding how the below decoding works...
                trace!("chunk {}: {:?}", i, chunk);

                // Starting a new row?
                if !chunk.row_key.is_empty() {
                    row_key = String::from_utf8(chunk.row_key).ok(); // Require UTF-8 for row keys
                }

                // Starting a new cell?
                if let Some(qualifier) = chunk.qualifier {
                    if let Some(cell_name) = cell_name {
                        row_data.push((cell_name, cell_value));
                        cell_value = vec![];
                    }
                    cell_name = String::from_utf8(qualifier).ok(); // Require UTF-8 for cell names
                    cell_timestamp = chunk.timestamp_micros;
                    cell_version_ok = true;
                } else {
                    // Continuing the existing cell.  Check if this is the start of another version of the cell
                    if chunk.timestamp_micros != 0 {
                        if chunk.timestamp_micros < cell_timestamp {
                            cell_version_ok = false; // ignore older versions of the cell
                        } else {
                            // newer version of the cell, remove the older cell
                            cell_version_ok = true;
                            cell_value = vec![];
                            cell_timestamp = chunk.timestamp_micros;
                        }
                    }
                }
                if cell_version_ok {
                    cell_value.append(&mut chunk.value);
                }

                // End of a row?
                if chunk.row_status.is_some() {
                    if let Some(RowStatus::CommitRow(_)) = chunk.row_status {
                        if let Some(cell_name) = cell_name {
                            row_data.push((cell_name, cell_value));
                        }

                        if let Some(row_key) = row_key {
                            rows.push((row_key, row_data))
                        }
                    }

                    row_key = None;
                    row_data = vec![];
                    cell_value = vec![];
                    cell_name = None;
                }
            }
        }
        Ok(rows)
    }
}
