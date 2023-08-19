use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde_json::json;
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};

use crate::{AppState, DEFAULT_TRACING_LEVEL};

/// Register the handlers and paths to router.
pub async fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route(
            state.config.server.graphql_url.as_str(),
            get(graphiql).post(graphql_handler),
        )
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

// GraphQL
// -------

/// GraphiQL handler.
/// GraphQL API webkit for dev testing.
/// It would be commented out in product mode.
pub async fn graphiql(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(
                format!(
                    "{}{}",
                    state.config.server.app_url, state.config.server.graphql_url
                )
                .as_str(),
            )
            .finish(),
    )
}

/// Handler for GraphQL requests.
pub async fn graphql_handler(
    State(state): State<Arc<AppState>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(req.into_inner()).await.into()
}
