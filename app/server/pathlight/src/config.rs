//! Defines the configuration structure for the application.
use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebsiteConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub admin_db: DatabaseConfig,
    pub website: WebsiteConfig,
}

impl Config {
    /// Loads the configuration from a TOML file and environment variables.
    pub fn from_env() -> anyhow::Result<Self> {
        let config: Config = Figment::new()
            .merge(Toml::file("app/server/pathlight/config/default.toml"))
            .merge(Env::prefixed("APP_").split("_"))
            .extract()?;
        Ok(config)
    }
}
