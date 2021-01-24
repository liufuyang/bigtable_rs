use crate::google::bigtable::v2::{
    bigtable_client::BigtableClient, read_rows_response::cell_chunk::RowStatus, ReadRowsRequest,
    ReadRowsResponse,
};

use crate::{
    access_token::{AccessToken, Scope},
    root_ca_certificate,
};
use log::{info, trace, warn};
use std::time::{Duration, Instant};
use thiserror::Error;
use tonic::transport::Endpoint;
use tonic::{
    codec::Streaming, metadata::MetadataValue, transport::Channel, transport::ClientTlsConfig,
    Request,
};

pub type RowKey = Vec<u8>;
pub type CellValue = Vec<u8>;

pub struct RowCell {
    pub family_name: String,
    pub qualifier: Vec<u8>,
    pub value: CellValue,
    pub timestamp_micros: i64,
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
                    access_token: None,
                    channel: Channel::balance_list(endpoints.into_iter()),
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
                    .map(|ep| ep.keep_alive_while_idle(true))
                    .map(|ep| {
                        if let Some(timeout) = timeout {
                            ep.timeout(timeout)
                        } else {
                            ep
                        }
                    })
                    .collect();

                Ok(Self {
                    access_token: Some(access_token),
                    channel: Channel::balance_list(endpoints.into_iter()),
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
    pub table_prefix: String,
    timeout: Option<Duration>,
}

impl BigTable {
    pub async fn read_rows(
        &mut self,
        request: ReadRowsRequest,
    ) -> Result<Vec<(RowKey, Vec<RowCell>)>> {
        self.refresh_access_token().await;
        let response = self.client.read_rows(request).await?.into_inner();
        self.decode_read_rows_response(response).await
    }

    async fn refresh_access_token(&self) {
        if let Some(ref access_token) = self.access_token {
            access_token.refresh().await;
        }
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
            if let Some(timeout) = self.timeout {
                if Instant::now().duration_since(started) > timeout {
                    return Err(Error::TimeoutError(timeout.as_secs()));
                }
            }
            for (i, mut chunk) in res.chunks.into_iter().enumerate() {
                // The comments for `read_rows_response::CellChunk` provide essential details for
                // understanding how the below decoding works...
                trace!("chunk {}: {:?}", i, chunk);

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
