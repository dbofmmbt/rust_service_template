use tracing::{info, instrument};

use crate::api::api;

#[tokio::main]
async fn main() {
    setup::telemetry();
    start_server().await;
}

#[instrument]
async fn start_server() {
    info!("Server started!");

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(api().into_make_service())
        .await
        .unwrap();
}

mod api;
mod setup;
