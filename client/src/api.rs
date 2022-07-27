#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequest {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="14")]
    pub headers: ::prost::alloc::vec::Vec<Header>,
    #[prost(bytes="vec", optional, tag="15")]
    pub body: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(oneof="http_request::Method", tags="4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
    pub method: ::core::option::Option<http_request::Method>,
}
/// Nested message and enum types in `HttpRequest`.
pub mod http_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        #[prost(message, tag="4")]
        Get(super::Get),
        #[prost(message, tag="5")]
        Post(super::Post),
        #[prost(message, tag="6")]
        Put(super::Put),
        #[prost(message, tag="7")]
        Patch(super::Patch),
        #[prost(message, tag="8")]
        Delete(super::Delete),
        #[prost(message, tag="9")]
        Connect(super::Connect),
        #[prost(message, tag="10")]
        Options(super::Options),
        #[prost(message, tag="11")]
        Trace(super::Trace),
        #[prost(message, tag="12")]
        Head(super::Head),
        #[prost(string, tag="13")]
        Other(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Post {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Put {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Patch {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Delete {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connect {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Options {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trace {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Head {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpResponse {
    #[prost(uint32, tag="1")]
    pub status: u32,
    #[prost(message, repeated, tag="2")]
    pub headers: ::prost::alloc::vec::Vec<Header>,
    #[prost(bytes="vec", tag="3")]
    pub body: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(bytes="vec", tag="1")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApiClient<tonic::transport::Channel> {
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
    impl<T> ApiClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ApiClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            ApiClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn call(
            &mut self,
            request: impl tonic::IntoRequest<super::HttpRequest>,
        ) -> Result<tonic::Response<super::HttpResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Api/call");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
