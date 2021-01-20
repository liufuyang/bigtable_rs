use crate::google::bigtable::v2::*;

use crate::{
    access_token::{AccessToken, Scope},
    root_ca_certificate,
};
use std::time::Duration;
use thiserror::Error;
use tonic::{metadata::MetadataValue, transport::ClientTlsConfig, Request};
use log::info;

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
                    timeout
                })
            }
        }
    }
}