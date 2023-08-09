use anyhow::{Ok, Result};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// The `Config` instance initiallization.
static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Unable to load config."));

/// Application server settings.
#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    /// the address to listen on.
    pub listen_addr: String,

    /// the port to listen on.
    pub listen_port: u16,
}

/// Application configuration settings.
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: Server,
}

impl Config {
    /// Create a new `Config` instance by merging the config files.
    /// The config files are loaded from the config directory.
    /// default.toml is loaded first.
    /// testing.toml for testing purposes. If default.toml exists, it would be overridden.
    pub fn new() -> Result<Self> {
        let config = Figment::new()
            .merge(Toml::file("config/default.toml"))
            .merge(Toml::file("config/testing.toml"))
            .extract()?;

        Ok(config)
    }
}

/// Get the initialized `Config` instance.
pub fn get_config() -> &'static Config {
    &CONFIG
}
