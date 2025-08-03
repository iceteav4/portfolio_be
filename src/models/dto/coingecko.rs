use crate::{models::common::asset::AssetImage, utils::coingecko::filter_market_data_by_currency};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct MarketData {
    pub current_price: HashMap<String, f64>,
    pub market_cap: HashMap<String, f64>,
    pub total_volume: HashMap<String, f64>,
    pub high_24h: HashMap<String, f64>,
    pub low_24h: HashMap<String, f64>,
}

impl MarketData {
    pub fn limit_as_currency(self) -> Self {
        Self {
            current_price: filter_market_data_by_currency(&self.current_price),
            market_cap: filter_market_data_by_currency(&self.market_cap),
            total_volume: filter_market_data_by_currency(&self.total_volume),
            high_24h: filter_market_data_by_currency(&self.high_24h),
            low_24h: filter_market_data_by_currency(&self.low_24h),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CoinDataResponse {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: Option<HashMap<String, String>>,
    pub image: AssetImage,
    pub market_data: MarketData,
}
