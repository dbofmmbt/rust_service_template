use tracing::{info, instrument};

use crate::{api::api, setup::config_setup};

#[tokio::main]
async fn main() {
    setup::telemetry();

    let settings = config_setup();

    println!("{:?}, {:?}", settings.debug, settings.text);

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
