use axum::{routing::get, Router};
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;

mod health;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::check))
        .layer(opentelemetry_tracing_layer())
}
