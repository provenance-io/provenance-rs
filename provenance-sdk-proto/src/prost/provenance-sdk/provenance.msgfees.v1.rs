/// MsgAssessCustomMsgFeeRequest defines an sdk.Msg type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAssessCustomMsgFeeRequest {
    /// optional short name for custom msg fee, this will be emitted as a property of the event
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// amount of additional fee that must be paid
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// optional recipient address, the basis points amount is sent to the recipient
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// the signer of the msg
    #[prost(string, tag = "4")]
    pub from: ::prost::alloc::string::String,
    /// optional basis points 0 - 10,000 for recipient defaults to 10,000
    #[prost(string, tag = "5")]
    pub recipient_basis_points: ::prost::alloc::string::String,
}
/// MsgAssessCustomMsgFeeResponse defines the Msg/AssessCustomMsgFeee response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAssessCustomMsgFeeResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the msgfees Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
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
        /// AssessCustomMsgFee endpoint executes the additional fee charges.
        /// This will only emit the event and not persist it to the keeper.  Fees are handled with the custom msg fee handlers
        /// Use Case: smart contracts will be able to charge additional fees and direct partial funds to specified recipient
        /// for executing contracts
        pub async fn assess_custom_msg_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAssessCustomMsgFeeRequest>,
        ) -> Result<tonic::Response<super::MsgAssessCustomMsgFeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.msgfees.v1.Msg/AssessCustomMsgFee",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// AssessCustomMsgFee endpoint executes the additional fee charges.
        /// This will only emit the event and not persist it to the keeper.  Fees are handled with the custom msg fee handlers
        /// Use Case: smart contracts will be able to charge additional fees and direct partial funds to specified recipient
        /// for executing contracts
        async fn assess_custom_msg_fee(
            &self,
            request: tonic::Request<super::MsgAssessCustomMsgFeeRequest>,
        ) -> Result<tonic::Response<super::MsgAssessCustomMsgFeeResponse>, tonic::Status>;
    }
    /// Msg defines the msgfees Msg service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/provenance.msgfees.v1.Msg/AssessCustomMsgFee" => {
                    #[allow(non_camel_case_types)]
                    struct AssessCustomMsgFeeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAssessCustomMsgFeeRequest>
                        for AssessCustomMsgFeeSvc<T>
                    {
                        type Response = super::MsgAssessCustomMsgFeeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAssessCustomMsgFeeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).assess_custom_msg_fee(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssessCustomMsgFeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "provenance.msgfees.v1.Msg";
    }
}
/// Params defines the set of params for the msgfees module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// constant used to calculate fees when gas fees shares denom with msg fee
    #[prost(message, optional, tag = "2")]
    pub floor_gas_price: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// total nhash per usd mil for converting usd to nhash
    #[prost(uint64, tag = "3")]
    pub nhash_per_usd_mil: u64,
    /// conversion fee denom is the denom usd is converted to
    #[prost(string, tag = "4")]
    pub conversion_fee_denom: ::prost::alloc::string::String,
}
/// MsgFee is the core of what gets stored on the blockchain
/// it consists of four parts
/// 1. the msg type url, i.e. /cosmos.bank.v1beta1.MsgSend
/// 2. minimum additional fees(can be of any denom)
/// 3. optional recipient of fee based on `recipient_basis_points`
/// 4. if recipient is declared they will recieve the basis points of the fee (0-10,000)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFee {
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// additional_fee can pay in any Coin( basically a Denom and Amount, Amount can be zero)
    #[prost(message, optional, tag = "2")]
    pub additional_fee: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// optional recipient address, the amount is split between recipient and fee module
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// optional split of funds between the recipient and fee module defaults to 50:50,
    #[prost(uint32, tag = "4")]
    pub recipient_basis_points: u32,
}
/// EventMsgFee final event property for msg fee on type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMsgFee {
    #[prost(string, tag = "1")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub count: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub total: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub recipient: ::prost::alloc::string::String,
}
/// EventMsgFees event emitted with summary of msg fees
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMsgFees {
    #[prost(message, repeated, tag = "1")]
    pub msg_fees: ::prost::alloc::vec::Vec<EventMsgFee>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAllMsgFeesRequest queries all Msg which have fees associated with them.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllMsgFeesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// response for querying all msg's with fees associated with them
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllMsgFeesResponse {
    #[prost(message, repeated, tag = "1")]
    pub msg_fees: ::prost::alloc::vec::Vec<MsgFee>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// CalculateTxFeesRequest is the request type for the Query RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateTxFeesRequest {
    /// tx_bytes is the transaction to simulate.
    #[prost(bytes = "vec", tag = "1")]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
    /// default_base_denom is used to set the denom used for gas fees
    /// if not set it will default to nhash.
    #[prost(string, tag = "2")]
    pub default_base_denom: ::prost::alloc::string::String,
    /// gas_adjustment is the adjustment factor to be multiplied against the estimate returned by the tx simulation
    #[prost(float, tag = "3")]
    pub gas_adjustment: f32,
}
/// CalculateTxFeesResponse is the response type for the Query RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalculateTxFeesResponse {
    /// additional_fees are the amount of coins to be for addition msg fees
    #[prost(message, repeated, tag = "1")]
    pub additional_fees: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// total_fees are the total amount of fees needed for the transactions (msg fees + gas fee)
    /// note: the gas fee is calculated with the floor gas price module param.
    #[prost(message, repeated, tag = "2")]
    pub total_fees: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// estimated_gas is the amount of gas needed for the transaction
    #[prost(uint64, tag = "3")]
    pub estimated_gas: u64,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service for marker module.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
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
        /// Params queries the parameters for x/msgfees
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.msgfees.v1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Query all Msgs which have fees associated with them.
        pub async fn query_all_msg_fees(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllMsgFeesRequest>,
        ) -> Result<tonic::Response<super::QueryAllMsgFeesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.msgfees.v1.Query/QueryAllMsgFees",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// CalculateTxFees simulates executing a transaction for estimating gas usage and additional fees.
        pub async fn calculate_tx_fees(
            &mut self,
            request: impl tonic::IntoRequest<super::CalculateTxFeesRequest>,
        ) -> Result<tonic::Response<super::CalculateTxFeesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.msgfees.v1.Query/CalculateTxFees",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// Params queries the parameters for x/msgfees
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        /// Query all Msgs which have fees associated with them.
        async fn query_all_msg_fees(
            &self,
            request: tonic::Request<super::QueryAllMsgFeesRequest>,
        ) -> Result<tonic::Response<super::QueryAllMsgFeesResponse>, tonic::Status>;
        /// CalculateTxFees simulates executing a transaction for estimating gas usage and additional fees.
        async fn calculate_tx_fees(
            &self,
            request: tonic::Request<super::CalculateTxFeesRequest>,
        ) -> Result<tonic::Response<super::CalculateTxFeesResponse>, tonic::Status>;
    }
    /// Query defines the gRPC querier service for marker module.
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/provenance.msgfees.v1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.msgfees.v1.Query/QueryAllMsgFees" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllMsgFeesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllMsgFeesRequest>
                        for QueryAllMsgFeesSvc<T>
                    {
                        type Response = super::QueryAllMsgFeesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllMsgFeesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_all_msg_fees(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllMsgFeesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.msgfees.v1.Query/CalculateTxFees" => {
                    #[allow(non_camel_case_types)]
                    struct CalculateTxFeesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::CalculateTxFeesRequest>
                        for CalculateTxFeesSvc<T>
                    {
                        type Response = super::CalculateTxFeesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CalculateTxFeesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).calculate_tx_fees(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CalculateTxFeesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "provenance.msgfees.v1.Query";
    }
}
/// GenesisState contains a set of msg fees, persisted from the store
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// msg_based_fees are the additional fees on specific tx msgs
    #[prost(message, repeated, tag = "2")]
    pub msg_fees: ::prost::alloc::vec::Vec<MsgFee>,
}
/// AddMsgFeeProposal defines a governance proposal to add additional msg based fee
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMsgFeeProposal {
    /// propsal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// propsal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// type url of msg to add fee
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// additional fee for msg type
    #[prost(message, optional, tag = "4")]
    pub additional_fee: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// optional recipient to recieve basis points
    #[prost(string, tag = "5")]
    pub recipient: ::prost::alloc::string::String,
    /// basis points to use when recipient is present (1 - 10,000)
    #[prost(string, tag = "6")]
    pub recipient_basis_points: ::prost::alloc::string::String,
}
/// UpdateMsgFeeProposal defines a governance proposal to update a current msg based fee
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMsgFeeProposal {
    /// propsal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// propsal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// type url of msg to update fee
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// additional fee for msg type
    #[prost(message, optional, tag = "4")]
    pub additional_fee: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// optional recipient to recieve basis points
    #[prost(string, tag = "5")]
    pub recipient: ::prost::alloc::string::String,
    /// basis points to use when recipient is present (1 - 10,000)
    #[prost(string, tag = "6")]
    pub recipient_basis_points: ::prost::alloc::string::String,
}
/// RemoveMsgFeeProposal defines a governance proposal to delete a current msg based fee
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMsgFeeProposal {
    /// propsal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// propsal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// type url of msg fee to remove
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// UpdateNhashPerUsdMilProposal defines a governance proposal to update the nhash per usd mil param
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNhashPerUsdMilProposal {
    /// proposal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// proposal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// nhash_per_usd_mil is number of nhash per usd mil
    #[prost(uint64, tag = "3")]
    pub nhash_per_usd_mil: u64,
}
/// UpdateConversionFeeDenomProposal defines a governance proposal to update the msg fee conversion denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConversionFeeDenomProposal {
    /// proposal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// proposal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// conversion_fee_denom is the denom that usd will be converted to
    #[prost(string, tag = "4")]
    pub conversion_fee_denom: ::prost::alloc::string::String,
}
