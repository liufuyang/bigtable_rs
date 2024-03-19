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
//!     let project_id = "project-1";
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

use std::sync::Arc;
use std::time::Duration;

use gcp_auth::AuthenticationManager;
use log::info;
use thiserror::Error;
use tonic::transport::Endpoint;
use tonic::{codec::Streaming, transport::Channel, transport::ClientTlsConfig, Response};
use tower::ServiceBuilder;

use crate::auth_service::AuthSvc;
use crate::bigtable::read_rows::decode_read_rows_response;
use crate::google::bigtable::v2::row_range::{EndKey, StartKey};
use crate::google::bigtable::v2::{
    bigtable_client::BigtableClient, MutateRowRequest, MutateRowResponse, MutateRowsRequest,
    MutateRowsResponse, ReadRowsRequest, RowRange, RowSet, SampleRowKeysRequest,
    SampleRowKeysResponse,
};
use crate::google::bigtable::v2::{CheckAndMutateRowRequest, CheckAndMutateRowResponse};
use crate::{root_ca_certificate, util::get_end_key};

pub mod read_rows;

/// An alias for Vec<u8> as row key
type RowKey = Vec<u8>;
/// A convenient Result type
type Result<T> = std::result::Result<T, Error>;

/// A data structure for returning the read content of a cell in a row.
#[derive(Debug)]
pub struct RowCell {
    pub family_name: String,
    pub qualifier: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp_micros: i64,
    pub labels: Vec<String>,
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

    #[error("Chunk error")]
    ChunkError(String),

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

    #[error("GCPAuthError error: {0}")]
    GCPAuthError(#[from] gcp_auth::Error),
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
    client: BigtableClient<AuthSvc>,
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
        is_read_only: bool,
        channel_size: usize,
        timeout: Option<Duration>,
    ) -> Result<Self> {
        let authentication_manager = AuthenticationManager::new().await?;
        Self::new_with_auth_manager(
            project_id,
            instance_name,
            is_read_only,
            channel_size,
            timeout,
            authentication_manager,
        )
        .await
    }
    /// Establish a connection to the BigTable instance named `instance_name`.  If read-only access
    /// is required, the `read_only` flag should be used to reduce the requested OAuth2 scope.
    ///
    /// The `authentication_manager` variable will be used to determine the
    /// program name that contains the BigTable instance in addition to access credentials.
    ///
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
    pub async fn new_with_auth_manager(
        project_id: &str,
        instance_name: &str,
        is_read_only: bool,
        channel_size: usize,
        timeout: Option<Duration>,
        authentication_manager: AuthenticationManager,
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
                    client: create_client(endpoints, None, is_read_only),
                    table_prefix: Arc::new(format!(
                        "projects/{}/instances/{}/tables/",
                        project_id, instance_name
                    )),
                    timeout: Arc::new(timeout),
                })
            }

            Err(_) => {
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

                let auth_manager = Some(Arc::new(authentication_manager));
                Ok(Self {
                    client: create_client(endpoints, auth_manager, is_read_only),
                    table_prefix: Arc::new(table_prefix),
                    timeout: Arc::new(timeout),
                })
            }
        }
    }

    /// Create a new BigTable client by cloning needed properties.
    ///
    /// Clients require `&mut self`, due to `Tonic::transport::Channel` limitations, however
    /// the created new clients can be cheaply cloned and thus can be send to different threads
    pub fn client(&self) -> BigTable {
        BigTable {
            client: self.client.clone(),
            table_prefix: self.table_prefix.clone(),
            timeout: self.timeout.clone(),
        }
    }
}

/// Helper function to create a BigtableClient<AuthSvc>
fn create_client(
    endpoints: Vec<Endpoint>,
    authentication_manager: Option<Arc<AuthenticationManager>>,
    read_only: bool,
) -> BigtableClient<AuthSvc> {
    let scopes = if read_only {
        "https://www.googleapis.com/auth/bigtable.data.readonly".to_string()
    } else {
        "https://www.googleapis.com/auth/bigtable.data".to_string()
    };
    let channel = Channel::balance_list(endpoints.into_iter());
    let auth_svc = ServiceBuilder::new()
        .layer_fn(|c| AuthSvc::new(c, authentication_manager.clone(), scopes.clone()))
        .service(channel);
    return BigtableClient::new(auth_svc);
}

/// The core struct for Bigtable client, which wraps a gPRC client defined by Bigtable proto.
/// In order to easily use this struct in multiple threads, we only store cloneable references here.
/// `BigtableClient<AuthSvc>` is a type alias of `BigtableClient` and it wraps a tonic Channel.
/// Cloning on `Bigtable` is cheap.
///
/// Bigtable can be created via `bigtable::BigTableConnection::new()` and cloned
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///   use bigtable_rs::bigtable;
///   let connection = bigtable::BigTableConnection::new("p-id", "i-id", true, 1, None).await?;
///   let bt_client = connection.client();
///   // Cheap to clone clients and used in other places.
///   let bt_client2 = bt_client.clone();
///   Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct BigTable {
    // clone is cheap with Channel, see https://docs.rs/tonic/latest/tonic/transport/struct.Channel.html
    client: BigtableClient<AuthSvc>,
    table_prefix: Arc<String>,
    timeout: Arc<Option<Duration>>,
}

impl BigTable {
    /// Wrapped `check_and_mutate_row` method
    pub async fn check_and_mutate_row(
        &mut self,
        request: CheckAndMutateRowRequest,
    ) -> Result<CheckAndMutateRowResponse> {
        let response = self
            .client
            .check_and_mutate_row(request)
            .await?
            .into_inner();
        Ok(response)
    }

    /// Wrapped `read_rows` method
    pub async fn read_rows(
        &mut self,
        request: ReadRowsRequest,
    ) -> Result<Vec<(RowKey, Vec<RowCell>)>> {
        let response = self.client.read_rows(request).await?.into_inner();
        decode_read_rows_response(self.timeout.as_ref(), response).await
    }

    /// Provide `read_rows_with_prefix` method to allow using a prefix as key
    pub async fn read_rows_with_prefix(
        &mut self,
        mut request: ReadRowsRequest,
        prefix: Vec<u8>,
    ) -> Result<Vec<(RowKey, Vec<RowCell>)>> {
        let end_key = get_end_key(prefix.as_ref()).map(|end_key| EndKey::EndKeyOpen(end_key));
        request.rows = Some(RowSet {
            row_keys: vec![], // use this field to put keys for reading specific rows
            row_ranges: vec![RowRange {
                start_key: Some(StartKey::StartKeyClosed(prefix)),
                end_key,
            }],
        });
        let response = self.client.read_rows(request).await?.into_inner();
        decode_read_rows_response(self.timeout.as_ref(), response).await
    }

    /// Wrapped `sample_row_keys` method
    pub async fn sample_row_keys(
        &mut self,
        request: SampleRowKeysRequest,
    ) -> Result<Streaming<SampleRowKeysResponse>> {
        let response = self.client.sample_row_keys(request).await?.into_inner();
        Ok(response)
    }

    /// Wrapped `mutate_row` method
    pub async fn mutate_row(
        &mut self,
        request: MutateRowRequest,
    ) -> Result<Response<MutateRowResponse>> {
        let response = self.client.mutate_row(request).await?;
        Ok(response)
    }

    /// Wrapped `mutate_rows` method
    pub async fn mutate_rows(
        &mut self,
        request: MutateRowsRequest,
    ) -> Result<Streaming<MutateRowsResponse>> {
        let response = self.client.mutate_rows(request).await?.into_inner();
        Ok(response)
    }

    /// Provide a convenient method to get the inner `BigtableClient` so user can use any methods
    /// defined from the Bigtable V2 gRPC API
    pub fn get_client(&mut self) -> &mut BigtableClient<AuthSvc> {
        &mut self.client
    }

    /// Provide a convenient method to get full table, which can be used for building requests
    pub fn get_full_table_name(&self, table_name: &str) -> String {
        [&self.table_prefix, table_name].concat()
    }
}
