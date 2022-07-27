pub mod api;

use std::error::Error;

use headers::{HeaderMap, HeaderName, HeaderValue};
use http::{
    header::{InvalidHeaderName, InvalidHeaderValue},
    Method,
};
use reqwest::Url;
use tonic::async_trait;

use api::{
    api_server::{Api, ApiServer},
    Header, HttpRequest, HttpResponse,
};

pub struct Server {
    inner: reqwest::Client,
}

impl Server {
    pub fn new(client: reqwest::Client) -> ApiServer<Server> {
        ApiServer::new(Server { inner: client })
    }
}

#[async_trait]
impl Api for Server {
    async fn call(
        &self,
        request: tonic::Request<HttpRequest>,
    ) -> Result<tonic::Response<HttpResponse>, tonic::Status> {
        let HttpRequest {
            url,
            path,
            query,
            method,
            headers,
            body,
        } = request.into_inner();
        let method = method
            .ok_or_else(|| tonic::Status::invalid_argument("method is required"))
            .and_then(Method::try_from)
            .map_err(|e| {
                tracing::info!("return bad method error");
                e
            })?;
        let mut url = Url::parse(&url).map_err(|e| {
            tracing::info!("return bad url error");
            tonic::Status::invalid_argument("bad url")
        })?;
        if !url.path().is_empty() && url.path() != "/" {
            return Err(tonic::Status::invalid_argument(
                "URL may only contain scheme and authority, provide path explicitly",
            ));
        }
        url.set_path(&path);
        url.set_query(query.as_deref());
        let mut req = self
            .inner
            .request(method, url)
            .headers(headers.into_iter().collect::<Result<_, HeaderError>>()?);
        if let Some(body) = body {
            req = req.body(body);
        }
        let req = req.build().map_err(|e| {
            tracing::warn!("Uncaught error while building request: {}", e);
            tonic::Status::invalid_argument("bad request")
        })?;
        let resp = self.inner.execute(req).await.map_err(|e| {
            if e.is_connect() {
                tracing::debug!("Failed executing request: {}", e);
                tonic::Status::unavailable("failed to connect to target endpoint")
            } else if e.is_timeout() {
                tracing::debug!("timeout while executing request: {}", e);
                tonic::Status::unavailable("request timed out")
            } else {
                tracing::warn!("unexpectedly failed executing request: {}", e);
                tonic::Status::unknown("failed to execute request")
            }
        })?;
        let headers = resp.headers().clone();
        let status = resp.status();
        let body = resp.bytes().await.map_err(|e| {
            tracing::warn!("failed decoding response: {}", e);
            if e.is_decode() {
                tonic::Status::unknown("failed to decode request body")
            } else {
                tonic::Status::internal("failed to receive body")
            }
        })?;

        Ok(tonic::Response::new(HttpResponse {
            status: status.as_u16() as _,
            headers: headers
                .iter()
                .map(|(k, v)| Header {
                    name: k.as_str().as_bytes().to_vec(),
                    value: v.as_bytes().to_vec(),
                })
                .collect(),
            body: body.into(),
        }))
    }
}

impl FromIterator<api::Header> for Result<HeaderMap, HeaderError> {
    fn from_iter<T: IntoIterator<Item = api::Header>>(iter: T) -> Self {
        let mut ret = HeaderMap::new();
        for api::Header { name, value } in iter {
            let key = HeaderName::from_bytes(&name)?;
            let value = HeaderValue::from_maybe_shared(value)?;
            ret.append(key, value);
        }
        Ok(ret)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum HeaderError {
    #[error(transparent)]
    InvalidName(#[from] InvalidHeaderName),
    #[error(transparent)]
    InvalidValue(#[from] InvalidHeaderValue),
}

impl From<HeaderError> for tonic::Status {
    fn from(_err: HeaderError) -> Self {
        tonic::Status::invalid_argument("bad header")
    }
}

impl TryFrom<api::http_request::Method> for Method {
    type Error = tonic::Status;

    fn try_from(value: api::http_request::Method) -> Result<Self, Self::Error> {
        match value {
            api::http_request::Method::Get(_) => Ok(Method::GET),
            api::http_request::Method::Post(_) => Ok(Method::POST),
            api::http_request::Method::Put(_) => Ok(Method::PUT),
            api::http_request::Method::Patch(_) => Ok(Method::PATCH),
            api::http_request::Method::Delete(_) => Ok(Method::DELETE),
            api::http_request::Method::Connect(_) => Ok(Method::CONNECT),
            api::http_request::Method::Trace(_) => Ok(Method::TRACE),
            api::http_request::Method::Options(_) => Ok(Method::OPTIONS),
            api::http_request::Method::Head(_) => Ok(Method::HEAD),
            api::http_request::Method::Other(o) => Method::from_bytes(o.as_bytes())
                .map_err(|_| tonic::Status::invalid_argument("bad method")),
        }
    }
}
