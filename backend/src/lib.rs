//! GraphQL API server library entry point.
#![forbid(unsafe_code)]

mod router;

use std::{net::TcpListener, sync::Arc};

use anyhow::{Ok, Result};
use axum::{routing::IntoMakeService, Router, Server};
use axum_graphql_utils::config::Config;
use hyper::server::conn::AddrIncoming;
use router::get_router;

pub static DEFAULT_TRACING_LEVEL: tracing::Level = tracing::Level::INFO;

/// Application state.
pub struct AppState {
    /// The application's configuration.
    pub config: &'static Config,
}

impl AppState {
    /// Create a new application state instance with config file.
    pub async fn new(config: &'static Config) -> Result<Self> {
        Ok(Self { config })
    }
}

/// Run the application server.
pub async fn run(state: Arc<AppState>) -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    let route = get_router(state.clone()).await;

    let addr = format!(
        "{}:{}",
        state.config.server.listen_addr, state.config.server.listen_port
    );

    let listener =
        TcpListener::bind(&addr).unwrap_or_else(|_| panic!("Unable to bind to {}", addr));

    let server = Server::from_tcp(listener)?.serve(route.into_make_service());

    Ok(server)
}
