use axum::{response::IntoResponse, Json};
use serde_json::json;

#[tracing::instrument]
pub async fn check() -> impl IntoResponse {
    tracing::info!("status checked");

    Json(json!(
        {"status": "up" }
    ))
}
