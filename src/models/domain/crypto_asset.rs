use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    models::{
        common::asset::{AssetImage, AssetType},
        dto::coingecko::CoinDataResponse,
    },
    utils::asset::get_asset_id,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCryptoAsset {
    pub id: String,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub external_id: String,
    pub platform_contract_map: HashMap<String, String>,
    pub image: AssetImage,
}

impl CreateCryptoAsset {
    pub fn from_coin_data(coin_data: CoinDataResponse) -> Self {
        let platform_contract_map = match coin_data.platforms {
            Some(v) => v,
            None => HashMap::new(),
        };
        Self {
            id: get_asset_id(AssetType::Crypto, &coin_data.id),
            source: "coingecko".into(),
            symbol: coin_data.symbol,
            name: coin_data.name,
            external_id: coin_data.id,
            platform_contract_map,
            image: coin_data.image,
        }
    }
}
