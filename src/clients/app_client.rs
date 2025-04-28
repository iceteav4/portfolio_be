use crate::config::ClientsConfig;

use super::coingecko::CoinGeckoClient;

#[derive(Clone)]
pub struct AppClients {
    pub coingecko: CoinGeckoClient,
}

impl AppClients {
    pub fn new(config: &ClientsConfig) -> Self {
        let coingecko = CoinGeckoClient::new(config.coingecko.api_key.clone());
        Self { coingecko }
    }
}
