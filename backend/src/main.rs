//! GraphQL API server.
#![forbid(unsafe_code)]

use graphql_backend::DEFAULT_TRACING_LEVEL;
use std::env;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[macro_use]
extern crate tracing;

/// GraphQL API backend application server entry point.
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_TRACING_LEVEL.to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting GraphQL API server...");
}
