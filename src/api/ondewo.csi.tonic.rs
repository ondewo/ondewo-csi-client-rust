// @generated
/// Generated client implementations.
pub mod conversations_client {
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
    pub struct ConversationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConversationsClient<tonic::transport::Channel> {
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
    impl<T> ConversationsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
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
        ) -> ConversationsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ConversationsClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_s2s_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::S2sPipeline>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.csi.Conversations/CreateS2sPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.csi.Conversations", "CreateS2sPipeline"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_s2s_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::S2sPipelineId>,
        ) -> std::result::Result<tonic::Response<super::S2sPipeline>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.csi.Conversations/GetS2sPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.csi.Conversations", "GetS2sPipeline"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_s2s_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::S2sPipeline>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.csi.Conversations/UpdateS2sPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.csi.Conversations", "UpdateS2sPipeline"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_s2s_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::S2sPipelineId>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.csi.Conversations/DeleteS2sPipeline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.csi.Conversations", "DeleteS2sPipeline"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_s2s_pipelines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListS2sPipelinesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2sPipelinesResponse>,
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
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.csi.Conversations/ListS2sPipelines",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.csi.Conversations", "ListS2sPipelines"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn s2s_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::S2sStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::S2sStreamResponse>>,
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
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.csi.Conversations/S2sStream",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.csi.Conversations", "S2sStream"));
            self.inner.streaming(req, path, codec).await
        }
        pub async fn check_upstream_health(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<super::CheckUpstreamHealthResponse>,
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
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.csi.Conversations/CheckUpstreamHealth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ondewo.csi.Conversations", "CheckUpstreamHealth"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_control_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::ControlStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ControlStreamResponse>>,
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
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.csi.Conversations/GetControlStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.csi.Conversations", "GetControlStream"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn set_control_status(
            &mut self,
            request: impl tonic::IntoRequest<super::SetControlStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetControlStatusResponse>,
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
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ondewo.csi.Conversations/SetControlStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ondewo.csi.Conversations", "SetControlStatus"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod conversations_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ConversationsServer.
    #[async_trait]
    pub trait Conversations: std::marker::Send + std::marker::Sync + 'static {
        async fn create_s2s_pipeline(
            &self,
            request: tonic::Request<super::S2sPipeline>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn get_s2s_pipeline(
            &self,
            request: tonic::Request<super::S2sPipelineId>,
        ) -> std::result::Result<tonic::Response<super::S2sPipeline>, tonic::Status>;
        async fn update_s2s_pipeline(
            &self,
            request: tonic::Request<super::S2sPipeline>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn delete_s2s_pipeline(
            &self,
            request: tonic::Request<super::S2sPipelineId>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        async fn list_s2s_pipelines(
            &self,
            request: tonic::Request<super::ListS2sPipelinesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListS2sPipelinesResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the S2sStream method.
        type S2sStreamStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::S2sStreamResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn s2s_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::S2sStreamRequest>>,
        ) -> std::result::Result<tonic::Response<Self::S2sStreamStream>, tonic::Status>;
        async fn check_upstream_health(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<super::CheckUpstreamHealthResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetControlStream method.
        type GetControlStreamStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ControlStreamResponse, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn get_control_stream(
            &self,
            request: tonic::Request<super::ControlStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::GetControlStreamStream>,
            tonic::Status,
        >;
        async fn set_control_status(
            &self,
            request: tonic::Request<super::SetControlStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetControlStatusResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ConversationsServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ConversationsServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ConversationsServer<T>
    where
        T: Conversations,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
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
                "/ondewo.csi.Conversations/CreateS2sPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct CreateS2sPipelineSvc<T: Conversations>(pub Arc<T>);
                    impl<
                        T: Conversations,
                    > tonic::server::UnaryService<super::S2sPipeline>
                    for CreateS2sPipelineSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::S2sPipeline>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Conversations>::create_s2s_pipeline(&inner, request)
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
                        let method = CreateS2sPipelineSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/ondewo.csi.Conversations/GetS2sPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct GetS2sPipelineSvc<T: Conversations>(pub Arc<T>);
                    impl<
                        T: Conversations,
                    > tonic::server::UnaryService<super::S2sPipelineId>
                    for GetS2sPipelineSvc<T> {
                        type Response = super::S2sPipeline;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::S2sPipelineId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Conversations>::get_s2s_pipeline(&inner, request)
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
                        let method = GetS2sPipelineSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/ondewo.csi.Conversations/UpdateS2sPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateS2sPipelineSvc<T: Conversations>(pub Arc<T>);
                    impl<
                        T: Conversations,
                    > tonic::server::UnaryService<super::S2sPipeline>
                    for UpdateS2sPipelineSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::S2sPipeline>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Conversations>::update_s2s_pipeline(&inner, request)
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
                        let method = UpdateS2sPipelineSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/ondewo.csi.Conversations/DeleteS2sPipeline" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteS2sPipelineSvc<T: Conversations>(pub Arc<T>);
                    impl<
                        T: Conversations,
                    > tonic::server::UnaryService<super::S2sPipelineId>
                    for DeleteS2sPipelineSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::S2sPipelineId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Conversations>::delete_s2s_pipeline(&inner, request)
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
                        let method = DeleteS2sPipelineSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/ondewo.csi.Conversations/ListS2sPipelines" => {
                    #[allow(non_camel_case_types)]
                    struct ListS2sPipelinesSvc<T: Conversations>(pub Arc<T>);
                    impl<
                        T: Conversations,
                    > tonic::server::UnaryService<super::ListS2sPipelinesRequest>
                    for ListS2sPipelinesSvc<T> {
                        type Response = super::ListS2sPipelinesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListS2sPipelinesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Conversations>::list_s2s_pipelines(&inner, request)
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
                        let method = ListS2sPipelinesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/ondewo.csi.Conversations/S2sStream" => {
                    #[allow(non_camel_case_types)]
                    struct S2sStreamSvc<T: Conversations>(pub Arc<T>);
                    impl<
                        T: Conversations,
                    > tonic::server::StreamingService<super::S2sStreamRequest>
                    for S2sStreamSvc<T> {
                        type Response = super::S2sStreamResponse;
                        type ResponseStream = T::S2sStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::S2sStreamRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Conversations>::s2s_stream(&inner, request).await
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
                        let method = S2sStreamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ondewo.csi.Conversations/CheckUpstreamHealth" => {
                    #[allow(non_camel_case_types)]
                    struct CheckUpstreamHealthSvc<T: Conversations>(pub Arc<T>);
                    impl<T: Conversations> tonic::server::UnaryService<()>
                    for CheckUpstreamHealthSvc<T> {
                        type Response = super::CheckUpstreamHealthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Conversations>::check_upstream_health(&inner, request)
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
                        let method = CheckUpstreamHealthSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/ondewo.csi.Conversations/GetControlStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetControlStreamSvc<T: Conversations>(pub Arc<T>);
                    impl<
                        T: Conversations,
                    > tonic::server::ServerStreamingService<super::ControlStreamRequest>
                    for GetControlStreamSvc<T> {
                        type Response = super::ControlStreamResponse;
                        type ResponseStream = T::GetControlStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ControlStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Conversations>::get_control_stream(&inner, request)
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
                        let method = GetControlStreamSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/ondewo.csi.Conversations/SetControlStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SetControlStatusSvc<T: Conversations>(pub Arc<T>);
                    impl<
                        T: Conversations,
                    > tonic::server::UnaryService<super::SetControlStatusRequest>
                    for SetControlStatusSvc<T> {
                        type Response = super::SetControlStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetControlStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Conversations>::set_control_status(&inner, request)
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
                        let method = SetControlStatusSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
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
    impl<T> Clone for ConversationsServer<T> {
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
    pub const SERVICE_NAME: &str = "ondewo.csi.Conversations";
    impl<T> tonic::server::NamedService for ConversationsServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
