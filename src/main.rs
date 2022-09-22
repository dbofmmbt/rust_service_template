use tracing::{info, instrument};

use crate::{api::api, setup::config_setup};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    setup::telemetry()?;

    let settings = config_setup()?;

    info!(settings.debug, settings.text, "config loaded");

    start_server().await
}

#[instrument]
async fn start_server() -> eyre::Result<()> {
    info!("Server started!");

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(api().into_make_service())
        .await?;
    Ok(())
}

mod api;
mod setup;
