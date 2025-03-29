use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::{models::dto::coingecko::CoinDataResponse, utils::error::AppError};

const BASE_URL: &str = "https://api.coingecko.com/api/v3";
const API_HEADER: &str = "x-cg-demo-api-key";

#[derive(Clone)]
pub struct CoinGeckoClient {
    base_url: String,
    res_client: Client,
    headers: HeaderMap,
}

impl CoinGeckoClient {
    pub fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(API_HEADER, HeaderValue::from_str(&api_key).unwrap());
        Self {
            base_url: BASE_URL.to_string(),
            res_client: Client::new(),
            headers,
        }
    }

    pub async fn get_coin_data(&self, coin_id: &str) -> Result<CoinDataResponse, AppError> {
        let response = self
            .res_client
            .get(format!("{}/coins/{}", self.base_url, coin_id))
            .headers(self.headers.clone())
            .send()
            .await
            .map_err(|e| AppError::CoinGeckoError(e.to_string()))?;
        response
            .json::<CoinDataResponse>()
            .await
            .map_err(|e| AppError::CoinGeckoError(e.to_string()))
    }
}
