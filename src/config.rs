use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub server_port: u16,
    pub secret_key: String,
    pub redis_url: String,
}

pub fn load_config() -> Result<Settings, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name("settings.toml"))
        .build()
        .unwrap()
        .try_deserialize()
}
