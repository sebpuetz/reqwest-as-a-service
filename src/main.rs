use reqwest_as_a_service::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = reqwest::Client::new();
    tonic::transport::Server::builder()
        .add_service(Server::new(client))
        .serve("127.0.0.1:3000".parse()?)
        .await?;
    Ok(())
}

