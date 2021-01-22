use crate::google::bigtable::v2::{
    bigtable_client::BigtableClient, ReadRowsRequest, ReadRowsResponse,
};

use crate::{
    access_token::{AccessToken, Scope},
    root_ca_certificate,
};
use log::{info, warn};
use std::time::Duration;
use thiserror::Error;
use tonic::transport::Endpoint;
use tonic::{
    codec::Streaming, metadata::MetadataValue, transport::Channel, transport::ClientTlsConfig,
    Request, Response, Status,
};

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
        project_id: &str,
        instance_name: &str,
        read_only: bool,
        timeout: Option<Duration>,
        channel_size: usize,
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
        }
    }
}

pub struct BigTable {
    access_token: Option<AccessToken>,
    client: BigtableClient<tonic::transport::Channel>,
    pub table_prefix: String,
}

impl BigTable {
    pub async fn read_rows(
        &mut self,
        request: ReadRowsRequest,
    ) -> std::result::Result<Response<Streaming<ReadRowsResponse>>, Status> {
        self.refresh_access_token().await;
        self.client.read_rows(request).await
    }

    async fn refresh_access_token(&self) {
        if let Some(ref access_token) = self.access_token {
            access_token.refresh().await;
        }
    }
}
