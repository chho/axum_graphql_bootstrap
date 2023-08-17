//! GraphQL API server library entry point.
#![forbid(unsafe_code)]

pub mod graphql;
mod router;

use std::{net::TcpListener, sync::Arc};

use anyhow::{Ok, Result};
use axum::{routing::IntoMakeService, Router, Server};
use axum_graphql_utils::config::{get_config, Config};
use graphql::{get_schema, GraphQLSchema};
use hyper::server::conn::AddrIncoming;
use router::get_router;

#[macro_use]
extern crate tracing;

pub static DEFAULT_TRACING_LEVEL: tracing::Level = tracing::Level::INFO;

/// Application state.
pub struct AppState {
    /// The application's configuration.
    pub config: &'static Config,
    pub schema: GraphQLSchema,
}

impl AppState {
    /// Create a new application state instance with config file.
    pub async fn new() -> Result<Self> {
        // get the configuration.
        let config = get_config();

        // get the GraphQL schema.
        let schema = get_schema().await?;

        // initialize the database.
        info!("Initializing database connection...");
        let db_url = format!("{}://{}", config.database.db_type, config.database.db_conn);
        info!(db_url);

        Ok(Self { config, schema })
    }
}

/// Run the application server.
pub async fn run() -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    // application state initialization.
    let state = Arc::new(AppState::new().await?);

    let mut route = get_router(state.clone()).await;

    // check the app_url and graphql_url whether starts with '/'.
    if !state.config.server.app_url.is_empty() && !state.config.server.app_url.starts_with('/')
        || !state.config.server.graphql_url.starts_with('/')
    {
        panic!("the app_url and graphql_url must start with '/', please check your config file.");
    }

    // if the app_url is not empty, add it to the router.
    if !state.config.server.app_url.is_empty() {
        route = Router::new().nest(state.config.server.app_url.as_str(), route);
    }

    let addr = format!(
        "{}:{}",
        state.config.server.listen_addr, state.config.server.listen_port
    );

    let listener =
        TcpListener::bind(&addr).unwrap_or_else(|_| panic!("Unable to bind to {}", addr));

    let server = Server::from_tcp(listener)?.serve(route.into_make_service());

    info!(
        "Server will serve on http://{}{}",
        server.local_addr(),
        state.config.server.app_url
    );

    info!(
        "GraphQL debug interface will serve on http://{}{}{}",
        server.local_addr(),
        state.config.server.app_url,
        state.config.server.graphql_url
    );

    Ok(server)
}
