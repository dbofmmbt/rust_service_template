use crate::helpers::TestServer;

mod helpers;

#[tokio::test]
async fn health_works() -> eyre::Result<()> {
    let server = TestServer::start().await?;
    let response = server.get("/health").send().await?;

    assert!(response.status().is_success());
    Ok(())
}
