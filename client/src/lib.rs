mod api;

use api::{
    api_client::ApiClient, Connect, Delete, Get, HttpRequest, HttpResponse, Options, Patch, Post,
    Put, Trace, Head,
};
use http::{uri::PathAndQuery, Method, StatusCode, Uri};
use tonic::transport::Channel;

use crate::api::Header;

pub struct Client {
    inner: ApiClient<Channel>,
}

impl Client {
    pub fn new(inner: ApiClient<Channel>) -> Self {
        Self { inner }
    }

    pub async fn call(&mut self, req: http::Request<Option<Vec<u8>>>) -> http::Response<Vec<u8>> {
        let HttpResponse {
            status,
            headers,
            body,
        } = self
            .inner
            .call(HttpRequest::from(req))
            .await
            .unwrap()
            .into_inner();
        let mut parts = http::Response::builder().status(status as u16);
        for header in headers {
            parts = parts.header(header.name, header.value);
        }

        parts.body(body).unwrap()
    }
}

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = api::api_client::ApiClient::connect("http://localhost:3000").await?;
    let mut client = Client::new(client);
    let uri = Uri::builder()
        .scheme("https")
        .authority("google.com")
        .path_and_query("/")
        .build()?;
    let req = http::Request::builder()
        .uri(uri)
        .method(Method::POST)
        .body(Some(b"hi".to_vec()))?;
    let resp = client.call(req).await;
    println!("{}", resp.status());
    Ok(())
}

impl From<http::Request<Option<Vec<u8>>>> for HttpRequest {
    fn from(req: http::Request<Option<Vec<u8>>>) -> Self {
        let (parts, body) = req.into_parts();
        let path = parts.uri.path();
        let query = parts.uri.query().map(Into::into);
        let authority = parts.uri.authority().unwrap();
        let scheme = parts.uri.scheme().map(|s| s.as_str()).unwrap();
        let url = format!("{}://{}", scheme, authority);
        let headers = parts
            .headers
            .iter()
            .map(|(k, v)| Header {
                name: k.as_str().as_bytes().to_vec(),
                value: v.as_bytes().to_vec(),
            })
            .collect();
        HttpRequest {
            url,
            path: path.into(),
            query,
            method: Some(parts.method.into()),
            headers,
            body,
        }
    }
}

impl From<Method> for api::http_request::Method {
    fn from(value: Method) -> Self {
        match value.as_str() {
            "OPTIONS" => api::http_request::Method::Options(Options {}),
            "GET" => api::http_request::Method::Get(Get {}),
            "POST" => api::http_request::Method::Post(Post {}),
            "PUT" => api::http_request::Method::Put(Put {}),
            "DELETE" => api::http_request::Method::Delete(Delete {}),
            "HEAD" => api::http_request::Method::Head(Head {}),
            "TRACE" => api::http_request::Method::Trace(Trace {}),
            "CONNECT" => api::http_request::Method::Connect(Connect {}),
            "PATCH" => api::http_request::Method::Patch(Patch {}),
            o => api::http_request::Method::Other(o.into()),
        }
    }
}
