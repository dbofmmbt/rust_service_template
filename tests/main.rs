use wiremock::{matchers::path, Mock, MockServer, ResponseTemplate};

use crate::helpers::TestServer;

mod helpers;

#[tokio::test]
async fn health_works() -> eyre::Result<()> {
    let server = TestServer::start().await?;
    let response = server.get("/health").send().await?;

    assert!(response.status().is_success());
    Ok(())
}

#[tokio::test]
async fn wiremock_works() -> eyre::Result<()> {
    let mock_server = MockServer::start().await;

    Mock::given(path("/test"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = reqwest::Client::new();

    let response = client.get(mock_server.uri() + "/test").send().await?;

    assert!(response.status().is_success());
    Ok(())
}
