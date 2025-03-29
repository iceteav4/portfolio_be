use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
    pub secret_key: String,
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
pub struct Coingecko {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub postgres: Postgres,
    pub redis: Redis,
    pub coingecko: Coingecko,
}

pub fn load_config() -> Result<Settings, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name("settings.toml"))
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap()
        .try_deserialize()
}
