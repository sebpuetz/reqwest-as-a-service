use http::StatusCode;
use reqwest_as_a_service::api::{http_request, Get, HttpRequest};
use tokio_stream::wrappers::TcpListenerStream;
use wiremock::{matchers, Mock, MockServer, Request, ResponseTemplate};

#[tokio::test]
async fn test() {
    tracing_subscriber::fmt::init();

    let mock_srv = spawn_echo_mock().await;

    let service_service = reqwest_as_a_service::Server::new(Default::default());
    let incoming = tokio::net::TcpListener::bind(("0.0.0.0", 0u16))
        .await
        .unwrap();
    let addr = incoming.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(service_service)
            .serve_with_incoming(TcpListenerStream::new(incoming)),
    );
    let mut service_service_client =
        reqwest_as_a_service::api::api_client::ApiClient::connect(format!("http://{}", addr))
            .await
            .unwrap();
    let req = HttpRequest {
        url: format!("http://{}", mock_srv.address().to_string()),
        path: "".into(),
        query: None,
        method: Some(http_request::Method::Get(Get {})),
        headers: Vec::new(),
        body: None,
    };
    let resp = service_service_client.call(req).await.unwrap().into_inner();
    assert_eq!(resp.status, 200);
}

async fn spawn_echo_mock() -> MockServer {
    let srv = MockServer::builder().start().await;
    Mock::given(matchers::any())
        .respond_with(|req: &Request| {
            ResponseTemplate::new(StatusCode::OK).set_body_bytes(req.body.clone())
        })
        .expect(1)
        .mount(&srv)
        .await;
    srv
}
