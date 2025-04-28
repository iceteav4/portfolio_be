use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
    pub secret_key: String,
    pub allow_origins: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct CoingeckoConfig {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ClientsConfig {
    pub coingecko: CoingeckoConfig,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub logging: Logging,
    pub postgres: Postgres,
    pub redis: Redis,
    pub clients: ClientsConfig,
}

pub fn load_config() -> Result<Settings, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name("settings.toml"))
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap()
        .try_deserialize()
}
