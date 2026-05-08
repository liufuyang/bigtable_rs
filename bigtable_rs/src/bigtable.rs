//! `bigtable` module provides a few convenient structs for calling Google Bigtable from Rust code.
//!
//!
//! Example usage:
//! ```rust,no_run
//! use bigtable_rs::bigtable;
//! use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::row_filter::{Chain, Filter};
//! use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::row_range::{EndKey, StartKey};
//! use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::{ReadRowsRequest, RowFilter, RowRange, RowSet};
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

use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use futures_util::Stream;
use gcp_auth::TokenProvider;
use log::info;
use thiserror::Error;
use tokio::net::UnixStream;
use tonic::metadata::MetadataValue;
use tonic::transport::Endpoint;
use tonic::IntoRequest;
use tonic::{
    codec::Streaming,
    transport::{channel::Change, Channel, ClientTlsConfig},
    Response,
};
use tower::ServiceBuilder;

use crate::auth_service::AuthSvc;
use crate::bigtable::read_rows::{decode_read_rows_response, decode_read_rows_response_stream};
use crate::{root_ca_certificate, util::get_row_range_from_prefix};
use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::{
    bigtable_client::BigtableClient, MutateRowRequest, MutateRowResponse, MutateRowsRequest,
    MutateRowsResponse, ReadRowsRequest, RowSet, SampleRowKeysRequest, SampleRowKeysResponse,
};
use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::{
    CheckAndMutateRowRequest, CheckAndMutateRowResponse, ExecuteQueryRequest, ExecuteQueryResponse,
};

pub mod read_rows;

pub trait RoutingMetadata {
    fn get_routing_header(&self) -> String;
}

impl RoutingMetadata for ReadRowsRequest {
    fn get_routing_header(&self) -> String {
        format!(
            "table_name={}&app_profile_id={}",
            urlencoding::encode(&self.table_name),
            urlencoding::encode(&self.app_profile_id)
        )
    }
}

impl RoutingMetadata for MutateRowRequest {
    fn get_routing_header(&self) -> String {
        format!(
            "table_name={}&app_profile_id={}",
            urlencoding::encode(&self.table_name),
            urlencoding::encode(&self.app_profile_id)
        )
    }
}

impl RoutingMetadata for MutateRowsRequest {
    fn get_routing_header(&self) -> String {
        format!(
            "table_name={}&app_profile_id={}",
            urlencoding::encode(&self.table_name),
            urlencoding::encode(&self.app_profile_id)
        )
    }
}

impl RoutingMetadata for CheckAndMutateRowRequest {
    fn get_routing_header(&self) -> String {
        format!(
            "table_name={}&app_profile_id={}",
            urlencoding::encode(&self.table_name),
            urlencoding::encode(&self.app_profile_id)
        )
    }
}

impl RoutingMetadata for SampleRowKeysRequest {
    fn get_routing_header(&self) -> String {
        format!(
            "table_name={}&app_profile_id={}",
            urlencoding::encode(&self.table_name),
            urlencoding::encode(&self.app_profile_id)
        )
    }
}

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

    #[error("Invalid metadata")]
    MetadataError(tonic::metadata::errors::InvalidMetadataValue),
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
    instance_prefix: Arc<String>,
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
    /// service, and the requests are load balanced onto all the channels.
    /// Consult the [Bigtable
    /// docs](https://docs.cloud.google.com/bigtable/docs/configure-connection-pools) for guidance
    /// on how to determine the optimal pool size for your application.
    /// As documented in [Cold starts and low
    /// QPS](https://docs.cloud.google.com/bigtable/docs/performance#cold-starts), you should
    /// configure the pool size in a way that ensures all channels receive a steady amount of load
    /// at all times. Failure to do so could result in latency spikes, as the server closes
    /// connections after a period of inactivity.
    /// Another approach to address this is to periodically send a low rate of artificial traffic
    /// to the table at all times, to ensure no connection becomes idle.
    /// If you are not sure what value to pick and your load is low, just start with 1.
    pub async fn new(
        project_id: &str,
        instance_name: &str,
        is_read_only: bool,
        channel_size: usize,
        timeout: Option<Duration>,
    ) -> Result<Self> {
        match std::env::var("BIGTABLE_EMULATOR_HOST") {
            Ok(endpoint) => Self::new_with_emulator(
                endpoint.as_str(),
                project_id,
                instance_name,
                is_read_only,
                channel_size,
                timeout,
            ),

            Err(_) => {
                let token_provider = gcp_auth::provider().await?;
                Self::new_with_token_provider(
                    project_id,
                    instance_name,
                    is_read_only,
                    channel_size,
                    timeout,
                    token_provider,
                )
            }
        }
    }
    /// Establish a connection to the BigTable instance named `instance_name`. If read-only access
    /// is required, the `read_only` flag should be used to reduce the requested OAuth2 scope.
    ///
    /// The `authentication_manager` variable will be used to determine the
    /// program name that contains the BigTable instance in addition to access credentials.
    ///
    /// `channel_size` defines the number of connections (or channels) established to Bigtable
    /// service, and the requests are load balanced onto all the channels.
    /// Consult the [Bigtable
    /// docs](https://docs.cloud.google.com/bigtable/docs/configure-connection-pools) for guidance
    /// on how to determine the optimal pool size for your application.
    /// As documented in [Cold starts and low
    /// QPS](https://docs.cloud.google.com/bigtable/docs/performance#cold-starts), you should
    /// configure the pool size in a way that ensures all channels receive a steady amount of load
    /// at all times. Failure to do so could result in latency spikes, as the server closes
    /// connections after a period of inactivity.
    /// Another approach to address this is to periodically send a low rate of artificial traffic
    /// to the table at all times, to ensure no connection becomes idle.
    /// If you are not sure what value to pick and your load is low, just start with 1.
    pub fn new_with_token_provider(
        project_id: &str,
        instance_name: &str,
        is_read_only: bool,
        channel_size: usize,
        timeout: Option<Duration>,
        token_provider: Arc<dyn TokenProvider>,
    ) -> Result<Self> {
        match std::env::var("BIGTABLE_EMULATOR_HOST") {
            Ok(endpoint) => Self::new_with_emulator(
                endpoint.as_str(),
                project_id,
                instance_name,
                is_read_only,
                channel_size,
                timeout,
            ),

            Err(_) => {
                let instance_prefix = format!("projects/{project_id}/instances/{instance_name}");
                let table_prefix = format!("{instance_prefix}/tables/");

                let channel_size = channel_size.max(1);
                let (channel, tx) = Channel::balance_channel(channel_size);
                for i in 0..channel_size {
                    let endpoint = Channel::from_static("https://bigtable.googleapis.com")
                        .tls_config(
                            ClientTlsConfig::new()
                                .ca_certificate(
                                    root_ca_certificate::load()
                                        .map_err(Error::CertificateError)
                                        .expect("root certificate error"),
                                )
                                .domain_name("bigtable.googleapis.com"),
                        )
                        .map_err(Error::TransportError)?
                        .http2_keep_alive_interval(Duration::from_secs(30))
                        .keep_alive_timeout(Duration::from_secs(10))
                        .keep_alive_while_idle(true);

                    let endpoint = if let Some(timeout) = timeout {
                        endpoint.timeout(timeout)
                    } else {
                        endpoint
                    };

                    // Use unique keys to ensure each channel has a dedicated HTTP connection
                    tx.try_send(Change::Insert(i, endpoint)).unwrap();
                }

                let token_provider = Some(token_provider);
                Ok(Self {
                    client: create_client(channel, token_provider, is_read_only),
                    table_prefix: Arc::new(table_prefix),
                    instance_prefix: Arc::new(instance_prefix),
                    timeout: Arc::new(timeout),
                })
            }
        }
    }

    /// Establish a connection to a BigTable emulator at [emulator_endpoint].
    /// This is usually covered by [Self::new] or [Self::new_with_auth_manager],
    /// which both support the `BIGTABLE_EMULATOR_HOST` env variable. However,
    /// this function can also be used directly, in case setting
    /// `BIGTABLE_EMULATOR_HOST` is inconvenient.
    pub fn new_with_emulator(
        emulator_endpoint: &str,
        project_id: &str,
        instance_name: &str,
        is_read_only: bool,
        channel_size: usize,
        timeout: Option<Duration>,
    ) -> Result<Self> {
        info!("Connecting to bigtable emulator at {}", emulator_endpoint);

        // configures the endpoint with the specified parameters
        fn configure_endpoint(endpoint: Endpoint, timeout: Option<Duration>) -> Endpoint {
            let endpoint = endpoint
                .http2_keep_alive_interval(Duration::from_secs(30))
                .keep_alive_timeout(Duration::from_secs(10))
                .keep_alive_while_idle(true);

            if let Some(timeout) = timeout {
                endpoint.timeout(timeout)
            } else {
                endpoint
            }
        }

        // Parse emulator_endpoint. Officially, it's only host:port,
        // but unix:///path/to/unix.sock also works in the Go SDK at least.
        // Having the emulator listen on unix domain sockets without ip2unix is
        // covered in https://github.com/googleapis/google-cloud-go/pull/9665.
        let channel = if let Some(path) = emulator_endpoint.strip_prefix("unix://") {
            // the URL doesn't matter, we use a custom connector.
            let endpoint = Endpoint::from_static("http://[::]:50051");
            let endpoint = configure_endpoint(endpoint, timeout);

            let path: String = path.to_string();
            let connector = tower::service_fn({
                move |_: tonic::transport::Uri| {
                    let path = path.clone();
                    async move {
                        let stream = UnixStream::connect(path).await?;
                        Ok::<_, std::io::Error>(hyper_util::rt::TokioIo::new(stream))
                    }
                }
            });
            // TODO - somehow support channel_size for UDS here as well?
            endpoint.connect_with_connector_lazy(connector)
        } else {
            let channel_size = channel_size.max(1);
            let (channel, tx) = Channel::balance_channel(channel_size);
            for i in 0..channel_size {
                let endpoint = Channel::from_shared(format!("http://{}", emulator_endpoint))
                    .expect("invalid connection emulator uri");
                let endpoint = configure_endpoint(endpoint, timeout);

                // Use unique keys to ensure each channel has a dedicated HTTP connection
                tx.try_send(Change::Insert(i, endpoint)).unwrap();
            }
            channel
        };

        Ok(Self {
            client: create_client(channel, None, is_read_only),
            table_prefix: Arc::new(format!(
                "projects/{}/instances/{}/tables/",
                project_id, instance_name
            )),
            instance_prefix: Arc::new(format!(
                "projects/{}/instances/{}",
                project_id, instance_name
            )),
            timeout: Arc::new(timeout),
        })
    }

    /// Create a new BigTable client by cloning needed properties.
    ///
    /// Clients require `&mut self`, due to `Tonic::transport::Channel` limitations, however
    /// the created new clients can be cheaply cloned and thus can be send to different threads
    pub fn client(&self) -> BigTable {
        BigTable {
            client: self.client.clone(),
            instance_prefix: self.instance_prefix.clone(),
            table_prefix: self.table_prefix.clone(),
            timeout: self.timeout.clone(),
        }
    }

    /// Provide a convenient method to update the inner `BigtableClient` so a newly configured client can be set
    pub fn configure_inner_client(
        &mut self,
        config_fn: fn(BigtableClient<AuthSvc>) -> BigtableClient<AuthSvc>,
    ) {
        self.client = config_fn(self.client.clone());
    }
}

/// Helper function to create a BigtableClient<AuthSvc>
/// from a channel.
fn create_client(
    channel: Channel,
    token_provider: Option<Arc<dyn TokenProvider>>,
    read_only: bool,
) -> BigtableClient<AuthSvc> {
    let scopes = if read_only {
        "https://www.googleapis.com/auth/bigtable.data.readonly"
    } else {
        "https://www.googleapis.com/auth/bigtable.data"
    };

    let auth_svc = ServiceBuilder::new()
        .layer_fn(|c| AuthSvc::new(c, token_provider.clone(), scopes.to_string()))
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
    instance_prefix: Arc<String>,
    table_prefix: Arc<String>,
    timeout: Arc<Option<Duration>>,
}

impl BigTable {
    /// Wrapped `check_and_mutate_row` method
    pub async fn check_and_mutate_row(
        &mut self,
        request: CheckAndMutateRowRequest,
    ) -> Result<CheckAndMutateRowResponse> {
        let tonic_req = Self::add_routing_header(request.into_request())?;

        let response = self
            .client
            .check_and_mutate_row(tonic_req)
            .await?
            .into_inner();
        Ok(response)
    }

    /// Wrapped `read_rows` method
    pub async fn read_rows(
        &mut self,
        request: ReadRowsRequest,
    ) -> Result<Vec<(RowKey, Vec<RowCell>)>> {
        let tonic_req = Self::add_routing_header(request.into_request())?;
        let response = self.client.read_rows(tonic_req).await?.into_inner();
        decode_read_rows_response(self.timeout.as_ref(), response).await
    }

    /// Provide `read_rows_with_prefix` method to allow using a prefix as key
    pub async fn read_rows_with_prefix(
        &mut self,
        mut request: ReadRowsRequest,
        prefix: Vec<u8>,
    ) -> Result<Vec<(RowKey, Vec<RowCell>)>> {
        let row_range = get_row_range_from_prefix(prefix);
        request.rows = Some(RowSet {
            row_keys: vec![], // use this field to put keys for reading specific rows
            row_ranges: vec![row_range],
        });

        let tonic_req = Self::add_routing_header(request.into_request())?;
        let response = self.client.read_rows(tonic_req).await?.into_inner();
        decode_read_rows_response(self.timeout.as_ref(), response).await
    }

    /// Streaming support for `read_rows` method
    pub async fn stream_rows(
        &mut self,
        request: ReadRowsRequest,
    ) -> Result<impl Stream<Item = Result<(RowKey, Vec<RowCell>)>>> {
        let tonic_req = Self::add_routing_header(request.into_request())?;

        let response = self.client.read_rows(tonic_req).await?.into_inner();
        let stream = decode_read_rows_response_stream(response).await;
        Ok(stream)
    }

    /// Streaming support for `read_rows_with_prefix` method
    pub async fn stream_rows_with_prefix(
        &mut self,
        mut request: ReadRowsRequest,
        prefix: Vec<u8>,
    ) -> Result<impl Stream<Item = Result<(RowKey, Vec<RowCell>)>>> {
        let row_range = get_row_range_from_prefix(prefix);
        request.rows = Some(RowSet {
            row_keys: vec![],
            row_ranges: vec![row_range],
        });

        let tonic_req = Self::add_routing_header(request.into_request())?;

        let response = self.client.read_rows(tonic_req).await?.into_inner();
        let stream = decode_read_rows_response_stream(response).await;
        Ok(stream)
    }

    /// Wrapped `sample_row_keys` method
    pub async fn sample_row_keys(
        &mut self,
        request: SampleRowKeysRequest,
    ) -> Result<Streaming<SampleRowKeysResponse>> {
        let tonic_req = Self::add_routing_header(request.into_request())?;

        let response = self.client.sample_row_keys(tonic_req).await?.into_inner();
        Ok(response)
    }

    /// Wrapped `mutate_row` method
    pub async fn mutate_row(
        &mut self,
        request: MutateRowRequest,
    ) -> Result<Response<MutateRowResponse>> {
        let tonic_req = Self::add_routing_header(request.into_request())?;

        let response = self.client.mutate_row(tonic_req).await?;
        Ok(response)
    }

    /// Wrapped `mutate_rows` method
    pub async fn mutate_rows(
        &mut self,
        request: MutateRowsRequest,
    ) -> Result<Streaming<MutateRowsResponse>> {
        let tonic_req = Self::add_routing_header(request.into_request())?;

        let response = self.client.mutate_rows(tonic_req).await?.into_inner();
        Ok(response)
    }

    /// Wrapped `execute_query` method
    pub async fn execute_query(
        &mut self,
        request: ExecuteQueryRequest,
    ) -> Result<Streaming<ExecuteQueryResponse>> {
        let app_profile_id = request.app_profile_id.clone();
        let mut tonic_req: tonic::Request<_> = request.into_request();
        // Add x-goog-request-params header with routing options, without those the call fails.
        tonic_req.metadata_mut().insert(
            "x-goog-request-params",
            MetadataValue::from_str(&format!(
                "name={}&app_profile_id={}",
                self.instance_prefix, app_profile_id
            ))
            .map_err(Error::MetadataError)?,
        );
        let response = self.client.execute_query(tonic_req).await?.into_inner();
        Ok(response)
    }

    /// Provide a convenient method to get the inner `BigtableClient` so user can use any methods
    /// defined from the Bigtable V2 gRPC API
    pub fn get_client(&mut self) -> &mut BigtableClient<AuthSvc> {
        &mut self.client
    }

    /// Provide a convenient method to update the inner `BigtableClient` config
    pub fn configure_inner_client(
        &mut self,
        config_fn: fn(BigtableClient<AuthSvc>) -> BigtableClient<AuthSvc>,
    ) {
        self.client = config_fn(self.client.clone());
    }

    /// Provide a convenient method to get full table, which can be used for building requests
    pub fn get_full_table_name(&self, table_name: &str) -> String {
        [&self.table_prefix, table_name].concat()
    }

    /// Helper to attach `x-goog-request-params` header for table APIs
    fn add_routing_header<T: RoutingMetadata>(
        mut tonic_req: tonic::Request<T>,
    ) -> Result<tonic::Request<T>> {
        let header_val = tonic_req.get_ref().get_routing_header();
        tonic_req.metadata_mut().insert(
            "x-goog-request-params",
            MetadataValue::from_str(&header_val).map_err(Error::MetadataError)?,
        );
        Ok(tonic_req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    #[test]
    fn test_add_routing_header_standard() {
        let req = ReadRowsRequest {
            table_name: "projects/my-project/instances/my-instance/tables/my-table".to_owned(),
            app_profile_id: "default".to_owned(),
            ..ReadRowsRequest::default()
        };

        let tonic_req = Request::new(req);
        let result = BigTable::add_routing_header(tonic_req).expect("Failed to add routing header");

        let metadata = result.metadata();
        let header_val = metadata
            .get("x-goog-request-params")
            .expect("Header missing")
            .to_str()
            .expect("Header is not a valid string");

        let expected_table = "projects%2Fmy-project%2Finstances%2Fmy-instance%2Ftables%2Fmy-table";
        let expected = format!("table_name={}&app_profile_id=default", expected_table);

        assert_eq!(header_val, expected);
    }

    #[test]
    fn test_add_routing_header_empty_app_profile() {
        let req = ReadRowsRequest {
            table_name: "projects/my-project/instances/my-instance/tables/my-table".to_owned(),
            app_profile_id: "".to_owned(),
            ..ReadRowsRequest::default()
        };

        let tonic_req = Request::new(req);
        let result = BigTable::add_routing_header(tonic_req).expect("Failed to add routing header");

        let metadata = result.metadata();
        let header_val = metadata
            .get("x-goog-request-params")
            .expect("Header missing")
            .to_str()
            .expect("Header is not a valid string");

        let expected_table = "projects%2Fmy-project%2Finstances%2Fmy-instance%2Ftables%2Fmy-table";
        let expected = format!("table_name={}&app_profile_id=", expected_table);

        assert_eq!(header_val, expected);
    }

    #[test]
    fn test_add_routing_header_with_special_chars() {
        let req = ReadRowsRequest {
            table_name: "my table@name".to_owned(),
            app_profile_id: "profile/v1".to_owned(),
            ..ReadRowsRequest::default()
        };

        let tonic_req = Request::new(req);
        let result = BigTable::add_routing_header(tonic_req).unwrap();

        let metadata = result.metadata();
        let header_val = metadata
            .get("x-goog-request-params")
            .unwrap()
            .to_str()
            .unwrap();

        let expected = "table_name=my%20table%40name&app_profile_id=profile%2Fv1";

        assert_eq!(header_val, expected);
    }

    #[test]
    fn test_routing_metadata_trait() {
        let read_rows = ReadRowsRequest {
            table_name: "table1".to_owned(),
            app_profile_id: "profile1".to_owned(),
            ..Default::default()
        };
        assert_eq!(
            read_rows.get_routing_header(),
            "table_name=table1&app_profile_id=profile1"
        );

        let mutate_row = MutateRowRequest {
            table_name: "table2".to_owned(),
            app_profile_id: "profile2".to_owned(),
            ..Default::default()
        };
        assert_eq!(
            mutate_row.get_routing_header(),
            "table_name=table2&app_profile_id=profile2"
        );

        let sample_row_keys = SampleRowKeysRequest {
            table_name: "table3".to_owned(),
            app_profile_id: "profile3".to_owned(),
            ..Default::default()
        };
        assert_eq!(
            sample_row_keys.get_routing_header(),
            "table_name=table3&app_profile_id=profile3"
        );
    }
}
