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
/// Generated server implementations.
pub mod api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ApiServer.
    #[async_trait]
    pub trait Api: Send + Sync + 'static {
        async fn call(
            &self,
            request: tonic::Request<super::HttpRequest>,
        ) -> Result<tonic::Response<super::HttpResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ApiServer<T: Api> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Api> ApiServer<T> {
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ApiServer<T>
    where
        T: Api,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.Api/call" => {
                    #[allow(non_camel_case_types)]
                    struct callSvc<T: Api>(pub Arc<T>);
                    impl<T: Api> tonic::server::UnaryService<super::HttpRequest>
                    for callSvc<T> {
                        type Response = super::HttpResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HttpRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).call(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = callSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Api> Clone for ApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Api> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Api> tonic::transport::NamedService for ApiServer<T> {
        const NAME: &'static str = "api.Api";
    }
}
