// @generated
/// Generated client implementations.
pub mod bigtable_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BigtableClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BigtableClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BigtableClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BigtableClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            BigtableClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn read_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRowsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ReadRowsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/ReadRows",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.bigtable.v2.Bigtable", "ReadRows"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn sample_row_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::SampleRowKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SampleRowKeysResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/SampleRowKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.bigtable.v2.Bigtable", "SampleRowKeys"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn mutate_row(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateRowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MutateRowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/MutateRow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.bigtable.v2.Bigtable", "MutateRow"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mutate_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateRowsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MutateRowsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/MutateRows",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.bigtable.v2.Bigtable", "MutateRows"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn check_and_mutate_row(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckAndMutateRowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckAndMutateRowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/CheckAndMutateRow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.bigtable.v2.Bigtable", "CheckAndMutateRow"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn ping_and_warm(
            &mut self,
            request: impl tonic::IntoRequest<super::PingAndWarmRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PingAndWarmResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/PingAndWarm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.bigtable.v2.Bigtable", "PingAndWarm"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_modify_write_row(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadModifyWriteRowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadModifyWriteRowResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/ReadModifyWriteRow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.bigtable.v2.Bigtable", "ReadModifyWriteRow"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_initial_change_stream_partitions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GenerateInitialChangeStreamPartitionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::GenerateInitialChangeStreamPartitionsResponse,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/GenerateInitialChangeStreamPartitions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.bigtable.v2.Bigtable",
                        "GenerateInitialChangeStreamPartitions",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn read_change_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadChangeStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ReadChangeStreamResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/ReadChangeStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.bigtable.v2.Bigtable", "ReadChangeStream"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn execute_query(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ExecuteQueryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v2.Bigtable/ExecuteQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.bigtable.v2.Bigtable", "ExecuteQuery"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod bigtable_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BigtableServer.
    #[async_trait]
    pub trait Bigtable: std::marker::Send + std::marker::Sync + 'static {
        /// Server streaming response type for the ReadRows method.
        type ReadRowsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ReadRowsResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn read_rows(
            &self,
            request: tonic::Request<super::ReadRowsRequest>,
        ) -> std::result::Result<tonic::Response<Self::ReadRowsStream>, tonic::Status>;
        /// Server streaming response type for the SampleRowKeys method.
        type SampleRowKeysStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::SampleRowKeysResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn sample_row_keys(
            &self,
            request: tonic::Request<super::SampleRowKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SampleRowKeysStream>,
            tonic::Status,
        >;
        async fn mutate_row(
            &self,
            request: tonic::Request<super::MutateRowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MutateRowResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the MutateRows method.
        type MutateRowsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::MutateRowsResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn mutate_rows(
            &self,
            request: tonic::Request<super::MutateRowsRequest>,
        ) -> std::result::Result<tonic::Response<Self::MutateRowsStream>, tonic::Status>;
        async fn check_and_mutate_row(
            &self,
            request: tonic::Request<super::CheckAndMutateRowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckAndMutateRowResponse>,
            tonic::Status,
        >;
        async fn ping_and_warm(
            &self,
            request: tonic::Request<super::PingAndWarmRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PingAndWarmResponse>,
            tonic::Status,
        >;
        async fn read_modify_write_row(
            &self,
            request: tonic::Request<super::ReadModifyWriteRowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadModifyWriteRowResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GenerateInitialChangeStreamPartitions method.
        type GenerateInitialChangeStreamPartitionsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::GenerateInitialChangeStreamPartitionsResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn generate_initial_change_stream_partitions(
            &self,
            request: tonic::Request<super::GenerateInitialChangeStreamPartitionsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::GenerateInitialChangeStreamPartitionsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ReadChangeStream method.
        type ReadChangeStreamStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::ReadChangeStreamResponse,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn read_change_stream(
            &self,
            request: tonic::Request<super::ReadChangeStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ReadChangeStreamStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ExecuteQuery method.
        type ExecuteQueryStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ExecuteQueryResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn execute_query(
            &self,
            request: tonic::Request<super::ExecuteQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ExecuteQueryStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BigtableServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> BigtableServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BigtableServer<T>
    where
        T: Bigtable,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/google.bigtable.v2.Bigtable/ReadRows" => {
                    #[allow(non_camel_case_types)]
                    struct ReadRowsSvc<T: Bigtable>(pub Arc<T>);
                    impl<
                        T: Bigtable,
                    > tonic::server::ServerStreamingService<super::ReadRowsRequest>
                    for ReadRowsSvc<T> {
                        type Response = super::ReadRowsResponse;
                        type ResponseStream = T::ReadRowsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadRowsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::read_rows(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReadRowsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bigtable.v2.Bigtable/SampleRowKeys" => {
                    #[allow(non_camel_case_types)]
                    struct SampleRowKeysSvc<T: Bigtable>(pub Arc<T>);
                    impl<
                        T: Bigtable,
                    > tonic::server::ServerStreamingService<super::SampleRowKeysRequest>
                    for SampleRowKeysSvc<T> {
                        type Response = super::SampleRowKeysResponse;
                        type ResponseStream = T::SampleRowKeysStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SampleRowKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::sample_row_keys(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SampleRowKeysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bigtable.v2.Bigtable/MutateRow" => {
                    #[allow(non_camel_case_types)]
                    struct MutateRowSvc<T: Bigtable>(pub Arc<T>);
                    impl<
                        T: Bigtable,
                    > tonic::server::UnaryService<super::MutateRowRequest>
                    for MutateRowSvc<T> {
                        type Response = super::MutateRowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MutateRowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::mutate_row(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MutateRowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bigtable.v2.Bigtable/MutateRows" => {
                    #[allow(non_camel_case_types)]
                    struct MutateRowsSvc<T: Bigtable>(pub Arc<T>);
                    impl<
                        T: Bigtable,
                    > tonic::server::ServerStreamingService<super::MutateRowsRequest>
                    for MutateRowsSvc<T> {
                        type Response = super::MutateRowsResponse;
                        type ResponseStream = T::MutateRowsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MutateRowsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::mutate_rows(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MutateRowsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bigtable.v2.Bigtable/CheckAndMutateRow" => {
                    #[allow(non_camel_case_types)]
                    struct CheckAndMutateRowSvc<T: Bigtable>(pub Arc<T>);
                    impl<
                        T: Bigtable,
                    > tonic::server::UnaryService<super::CheckAndMutateRowRequest>
                    for CheckAndMutateRowSvc<T> {
                        type Response = super::CheckAndMutateRowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckAndMutateRowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::check_and_mutate_row(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CheckAndMutateRowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bigtable.v2.Bigtable/PingAndWarm" => {
                    #[allow(non_camel_case_types)]
                    struct PingAndWarmSvc<T: Bigtable>(pub Arc<T>);
                    impl<
                        T: Bigtable,
                    > tonic::server::UnaryService<super::PingAndWarmRequest>
                    for PingAndWarmSvc<T> {
                        type Response = super::PingAndWarmResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PingAndWarmRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::ping_and_warm(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PingAndWarmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bigtable.v2.Bigtable/ReadModifyWriteRow" => {
                    #[allow(non_camel_case_types)]
                    struct ReadModifyWriteRowSvc<T: Bigtable>(pub Arc<T>);
                    impl<
                        T: Bigtable,
                    > tonic::server::UnaryService<super::ReadModifyWriteRowRequest>
                    for ReadModifyWriteRowSvc<T> {
                        type Response = super::ReadModifyWriteRowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadModifyWriteRowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::read_modify_write_row(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReadModifyWriteRowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bigtable.v2.Bigtable/GenerateInitialChangeStreamPartitions" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateInitialChangeStreamPartitionsSvc<T: Bigtable>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Bigtable,
                    > tonic::server::ServerStreamingService<
                        super::GenerateInitialChangeStreamPartitionsRequest,
                    > for GenerateInitialChangeStreamPartitionsSvc<T> {
                        type Response = super::GenerateInitialChangeStreamPartitionsResponse;
                        type ResponseStream = T::GenerateInitialChangeStreamPartitionsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GenerateInitialChangeStreamPartitionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::generate_initial_change_stream_partitions(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GenerateInitialChangeStreamPartitionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bigtable.v2.Bigtable/ReadChangeStream" => {
                    #[allow(non_camel_case_types)]
                    struct ReadChangeStreamSvc<T: Bigtable>(pub Arc<T>);
                    impl<
                        T: Bigtable,
                    > tonic::server::ServerStreamingService<
                        super::ReadChangeStreamRequest,
                    > for ReadChangeStreamSvc<T> {
                        type Response = super::ReadChangeStreamResponse;
                        type ResponseStream = T::ReadChangeStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadChangeStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::read_change_stream(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReadChangeStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bigtable.v2.Bigtable/ExecuteQuery" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteQuerySvc<T: Bigtable>(pub Arc<T>);
                    impl<
                        T: Bigtable,
                    > tonic::server::ServerStreamingService<super::ExecuteQueryRequest>
                    for ExecuteQuerySvc<T> {
                        type Response = super::ExecuteQueryResponse;
                        type ResponseStream = T::ExecuteQueryStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteQueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Bigtable>::execute_query(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ExecuteQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for BigtableServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "google.bigtable.v2.Bigtable";
    impl<T> tonic::server::NamedService for BigtableServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
