//! GraphQL API server.
#![forbid(unsafe_code)]

use anyhow::{Ok, Result};
use axum_graphql_backend::{graphql::get_schema, run, AppState, DEFAULT_TRACING_LEVEL};
use axum_graphql_utils::config::get_config;
use std::{env, sync::Arc};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[macro_use]
extern crate tracing;

/// GraphQL API backend application server entry point.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_TRACING_LEVEL.to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = get_config();
    let schema = get_schema().await?;
    let state = Arc::new(AppState::new(config, schema).await?);

    let server = run(state).await?;

    info!("Server is starting on http://{}", server.local_addr());

    server.await?;

    Ok(())
}
