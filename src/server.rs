use std::net::TcpListener;

use tracing::{info, instrument};

use crate::routes::routes;

#[instrument(skip_all)]
pub async fn start(listener: TcpListener) -> eyre::Result<()> {
    info!("Server started!");

    axum::Server::from_tcp(listener)?
        .serve(routes().into_make_service())
        .await?;
    Ok(())
}
