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

use std::collections::HashMap;
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
    execute_query_response, prepare_query_request, r#type, result_set_metadata, value,
    CheckAndMutateRowRequest, CheckAndMutateRowResponse, ExecuteQueryRequest, ExecuteQueryResponse,
    PrepareQueryRequest, PrepareQueryResponse, ProtoFormat, ResultSetMetadata, Type, Value,
};

pub mod execute_query;
pub mod read_rows;

/// An alias for Vec<u8> as row key
type RowKey = Vec<u8>;
/// A convenient Result type
type Result<T> = std::result::Result<T, Error>;

/// A single row returned by [`BigTable::execute_query_stream`].
///
/// Values are in column order per `ResultSetMetadata.proto_schema.columns`.
#[derive(Debug, Clone)]
pub struct ResultSet(pub Vec<Value>);

/// A batch of rows from a single stream item, with an optional retry checkpoint.
///
/// Yielded by [`ExecuteQueryStream::next`]. `retry_context` is `Some` only at
/// checkpoint boundaries; pass it back to [`BigTable::execute_query_stream`] to
/// resume without re-reading earlier results.
#[derive(Debug)]
pub struct SqlQueryBatch {
    pub rows: Vec<ResultSet>,
    pub retry_context: Option<ExecuteQueryRetryContext>,
}

/// A stream of decoded rows from an [`BigTable::execute_query_stream`] call.
///
/// Drive with `.next().await` in a loop; no `futures-util` import required.
pub struct ExecuteQueryStream {
    inner: futures_util::stream::BoxStream<'static, Result<SqlQueryBatch>>,
}

impl ExecuteQueryStream {
    pub async fn next(&mut self) -> Option<Result<SqlQueryBatch>> {
        use futures_util::StreamExt;
        self.inner.next().await
    }
}

/// State needed to retry a streaming query from the last checkpoint.
///
/// Yielded by `execute_query_stream` stream items that carry a resume token.
/// Pass it back as `retry_context: Some(ctx)` to resume from that checkpoint
/// without re-reading earlier results.
#[derive(Debug, Clone)]
pub struct ExecuteQueryRetryContext {
    pub prepared_query: Vec<u8>,
    pub resume_token: Vec<u8>,
}

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

    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("CRC32C checksum mismatch: expected {expected:#010x}, got {actual:#010x}")]
    ChecksumMismatch { expected: u32, actual: u32 },

    #[error("Protocol violation: {0}")]
    ProtocolViolation(String),
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

// Inserts the x-goog-request-params routing header for SQL RPCs (PrepareQuery,
// ExecuteQuery).
fn insert_sql_routing_header(
    metadata: &mut tonic::metadata::MetadataMap,
    instance_prefix: &str,
    app_profile_id: &str,
) -> Result<()> {
    metadata.insert(
        "x-goog-request-params",
        MetadataValue::from_str(&format!(
            "name={}&app_profile_id={}",
            instance_prefix, app_profile_id
        ))
        .map_err(Error::MetadataError)?,
    );
    Ok(())
}

// Infers a Bigtable SQL Type from the kind of a Value. Explicit types always take precedence;
// this is only called when Value.r#type is None. Ambiguous kinds (float, array, NULL) require
// the caller to set r#type explicitly.
fn infer_value_type(name: &str, v: &Value) -> Result<Type> {
    let kind = match &v.kind {
        Some(value::Kind::BytesValue(_)) => r#type::Kind::BytesType(r#type::Bytes::default()),
        Some(value::Kind::StringValue(_)) => r#type::Kind::StringType(r#type::String::default()),
        Some(value::Kind::IntValue(_)) => r#type::Kind::Int64Type(r#type::Int64::default()),
        Some(value::Kind::BoolValue(_)) => r#type::Kind::BoolType(r#type::Bool::default()),
        Some(value::Kind::TimestampValue(_)) => {
            r#type::Kind::TimestampType(r#type::Timestamp::default())
        }
        Some(value::Kind::DateValue(_)) => r#type::Kind::DateType(r#type::Date::default()),
        Some(value::Kind::FloatValue(_)) => {
            return Err(Error::InvalidValue(format!(
                "param '{name}': cannot infer float type; set Value.r#type to Float32 or Float64 explicitly"
            )))
        }
        Some(value::Kind::ArrayValue(_)) => {
            return Err(Error::InvalidValue(format!(
                "param '{name}': cannot infer array element type; set Value.r#type explicitly"
            )))
        }
        Some(value::Kind::RawValue(_)) | Some(value::Kind::RawTimestampMicros(_)) => {
            return Err(Error::InvalidValue(format!(
                "param '{name}': raw values cannot be used as query parameters; use BytesValue or TimestampValue"
            )))
        }
        None => {
            return Err(Error::InvalidValue(format!(
                "param '{name}': NULL value requires an explicit Value.r#type"
            )))
        }
    };
    Ok(Type { kind: Some(kind) })
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
        let row_range = get_row_range_from_prefix(prefix);
        request.rows = Some(RowSet {
            row_keys: vec![], // use this field to put keys for reading specific rows
            row_ranges: vec![row_range],
        });
        let response = self.client.read_rows(request).await?.into_inner();
        decode_read_rows_response(self.timeout.as_ref(), response).await
    }

    /// Streaming support for `read_rows` method
    pub async fn stream_rows(
        &mut self,
        request: ReadRowsRequest,
    ) -> Result<impl Stream<Item = Result<(RowKey, Vec<RowCell>)>>> {
        let response = self.client.read_rows(request).await?.into_inner();
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
        let response = self.client.read_rows(request).await?.into_inner();
        let stream = decode_read_rows_response_stream(response).await;
        Ok(stream)
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

    // Validates and mutates `request` so it is ready to pass to the ExecuteQuery RPC:
    //   - Errors if both `query` and `prepared_query` are set, or if neither is set.
    //   - Errors if `resume_token` is set without `prepared_query`.
    //   - Calls `prepare_query` when only `query` is set, populating `prepared_query`.
    //   - Clears `data_format` (must be unset when `prepared_query` is used).
    #[allow(deprecated)]
    async fn ensure_prepared(&mut self, request: &mut ExecuteQueryRequest) -> Result<()> {
        if !request.prepared_query.is_empty() && !request.query.is_empty() {
            return Err(Error::InvalidValue(
                "cannot set both `query` and `prepared_query`; use one or the other".to_string(),
            ));
        }
        if request.prepared_query.is_empty() {
            if request.query.is_empty() {
                return Err(Error::InvalidValue(
                    "either `query` or `prepared_query` must be set".to_string(),
                ));
            }
            if !request.resume_token.is_empty() {
                return Err(Error::InvalidValue(
                    "resume_token requires prepared_query to be set; save the \
                     PrepareQueryResponse.prepared_query bytes from the first call \
                     and include them on retry"
                        .to_string(),
                ));
            }
            // Build param_types for PrepareQuery. If the caller set Value.r#type
            // explicitly, use it. Otherwise infer from the value kind, mirroring
            // the Python client. Ambiguous kinds (float, array, NULL) require an
            // explicit type.
            let param_types = request
                .params
                .iter()
                .map(|(k, v)| {
                    let t = if let Some(explicit) = v.r#type.clone() {
                        explicit
                    } else {
                        infer_value_type(k, v)?
                    };
                    Ok((k.clone(), t))
                })
                .collect::<Result<HashMap<_, _>>>()?;
            let query = std::mem::take(&mut request.query);
            let prepare_response = self
                .prepare_query(PrepareQueryRequest {
                    instance_name: request.instance_name.clone(),
                    app_profile_id: request.app_profile_id.clone(),
                    query,
                    param_types,
                    data_format: Some(prepare_query_request::DataFormat::ProtoFormat(
                        ProtoFormat {},
                    )),
                })
                .await?;
            request.prepared_query = prepare_response.prepared_query;
        }
        request.data_format = None;
        Ok(())
    }

    // Wrapped `prepare_query` method
    async fn prepare_query(
        &mut self,
        request: PrepareQueryRequest,
    ) -> Result<PrepareQueryResponse> {
        let app_profile_id = request.app_profile_id.clone();
        let mut tonic_req: tonic::Request<_> = request.into_request();
        insert_sql_routing_header(
            tonic_req.metadata_mut(),
            &self.instance_prefix,
            &app_profile_id,
        )?;
        let response = self.client.prepare_query(tonic_req).await?.into_inner();
        Ok(response)
    }

    // Sends the ExecuteQuery RPC and reads the leading ResultSetMetadata message.
    // Caller is responsible for setting prepared_query, resume_token, and data_format
    // before calling this.
    async fn do_execute_rpc(
        &mut self,
        request: ExecuteQueryRequest,
    ) -> Result<(ResultSetMetadata, Streaming<ExecuteQueryResponse>)> {
        let app_profile_id = request.app_profile_id.clone();
        let mut tonic_req: tonic::Request<_> = request.into_request();
        insert_sql_routing_header(
            tonic_req.metadata_mut(),
            &self.instance_prefix,
            &app_profile_id,
        )?;
        let mut stream = self.client.execute_query(tonic_req).await?.into_inner();

        let metadata = stream
            .message()
            .await?
            .ok_or_else(|| {
                Error::ProtocolViolation("execute_query returned an empty stream".to_string())
            })
            .and_then(|msg| match msg.response {
                Some(execute_query_response::Response::Metadata(m)) => Ok(m),
                _ => Err(Error::ProtocolViolation(
                    "first execute_query response was not ResultSetMetadata".to_string(),
                )),
            })?;

        Ok((metadata, stream))
    }

    /// Execute a SQL query and stream decoded rows as they become available.
    ///
    /// Pass `retry_context: None` for a fresh query (auto-prepares from `request.query`).
    /// Pass `retry_context: Some(ctx)` to resume from a checkpoint returned by a previous
    /// stream item; the prepared plan and resume token are taken from `ctx`. If
    /// `request.prepared_query` or `request.resume_token` are also set they must match
    /// the values in `ctx`; leaving them unset is equivalent.
    ///
    /// Returns `(ResultSetMetadata, Stream)` where each stream item is
    /// `(rows, retry_context)`:
    /// - `rows` is yielded as soon as a `batch_checksum` verifies the batch.
    /// - `retry_context` is `Some` at checkpoint boundaries; pass it back as
    ///   `retry_context` to resume from that point.
    ///
    /// Auto-prepare infers parameter types for unambiguous value kinds (`BytesValue`,
    /// `StringValue`, `IntValue`, `BoolValue`, `TimestampValue`, `DateValue`).
    /// Ambiguous kinds (`FloatValue`, `ArrayValue`, NULL) and raw values require an
    /// explicit `Value.r#type`.
    pub async fn execute_query_stream(
        &mut self,
        mut request: ExecuteQueryRequest,
        retry_context: Option<ExecuteQueryRetryContext>,
    ) -> Result<(ResultSetMetadata, ExecuteQueryStream)> {
        let prepared_query = match retry_context {
            None => {
                self.ensure_prepared(&mut request).await?;
                request.prepared_query.clone()
            }
            Some(ctx) => {
                #[allow(deprecated)]
                if !request.query.is_empty() {
                    return Err(Error::InvalidValue(
                        "when retry_context is Some, `query` must be unset on the request"
                            .to_string(),
                    ));
                }
                if ctx.prepared_query.is_empty() {
                    return Err(Error::InvalidValue(
                        "retry_context.prepared_query must not be empty".to_string(),
                    ));
                }
                if ctx.resume_token.is_empty() {
                    return Err(Error::InvalidValue(
                        "retry_context.resume_token must not be empty".to_string(),
                    ));
                }
                #[allow(deprecated)]
                if !request.prepared_query.is_empty()
                    && request.prepared_query != ctx.prepared_query
                {
                    return Err(Error::InvalidValue(
                        "request.prepared_query conflicts with retry_context.prepared_query"
                            .to_string(),
                    ));
                }
                #[allow(deprecated)]
                if !request.resume_token.is_empty() && request.resume_token != ctx.resume_token {
                    return Err(Error::InvalidValue(
                        "request.resume_token conflicts with retry_context.resume_token"
                            .to_string(),
                    ));
                }
                request.prepared_query = ctx.prepared_query.clone();
                request.resume_token = ctx.resume_token;
                request.data_format = None;
                ctx.prepared_query
            }
        };

        let (metadata, stream) = self.do_execute_rpc(request).await?;
        let num_columns = match &metadata.schema {
            Some(result_set_metadata::Schema::ProtoSchema(s)) => s.columns.len(),
            None => {
                return Err(Error::ProtocolViolation(
                    "ResultSetMetadata has no schema".to_string(),
                ))
            }
        };
        Ok((
            metadata,
            ExecuteQueryStream {
                inner: Box::pin(execute_query::make_stream(
                    stream,
                    num_columns,
                    prepared_query,
                )),
            },
        ))
    }

    /// Wrapped `execute_query` method.
    ///
    /// Auto-prepares if `prepared_query` is not set (see `execute_query_stream` for details).
    /// Returns the raw gRPC stream; prefer `execute_query_stream` for decoded rows.
    pub async fn execute_query(
        &mut self,
        mut request: ExecuteQueryRequest,
    ) -> Result<Streaming<ExecuteQueryResponse>> {
        self.ensure_prepared(&mut request).await?;
        let app_profile_id = request.app_profile_id.clone();
        let mut tonic_req: tonic::Request<_> = request.into_request();
        insert_sql_routing_header(
            tonic_req.metadata_mut(),
            &self.instance_prefix,
            &app_profile_id,
        )?;
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
}
