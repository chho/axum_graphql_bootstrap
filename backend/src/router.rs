use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Router};
use serde_json::json;
use tower_http::trace::{self, TraceLayer};

use crate::{AppState, DEFAULT_TRACING_LEVEL};

/// Register the handlers and paths to router.
pub async fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(DEFAULT_TRACING_LEVEL))
                .on_response(trace::DefaultOnResponse::new().level(DEFAULT_TRACING_LEVEL)),
        )
        .with_state(state)
}

// Health check
// ------------

/// Health check handler.
pub async fn health_check() -> impl IntoResponse {
    json!({"code":200,"success":true}).to_string()
}
