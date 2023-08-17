//! GraphQL API server.
#![forbid(unsafe_code)]

use anyhow::{Ok, Result};
use axum_graphql_backend::{run, DEFAULT_TRACING_LEVEL};
use std::env;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

/// GraphQL API backend application server entry point.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_TRACING_LEVEL.to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server = run().await?;

    server.await?;

    Ok(())
}
