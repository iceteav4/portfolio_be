use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{
    common::asset::{AssetExt, AssetImage, AssetType, CryptoExt},
    database::asset::AssetRow,
};

use super::coingecko::CoinDataResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct AssetResponse {
    pub id: String,
    pub asset_type: AssetType,
    pub symbol: String,
    pub name: String,
    pub image: AssetImage,
}

impl AssetResponse {
    pub fn from_row(row: AssetRow) -> Self {
        Self {
            id: row.id,
            asset_type: row.asset_type.parse().unwrap(),
            symbol: row.symbol,
            name: row.name,
            image: serde_json::from_value(row.image.clone()).unwrap(),
        }
    }

    pub fn from_asset_row(row: AssetRow) -> Self {
        Self {
            id: row.id.to_string(),
            asset_type: row.asset_type.parse().unwrap(),
            symbol: row.symbol,
            name: row.name,
            image: serde_json::from_value(row.image.clone()).unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AssetQueryParams {
    pub asset_type: Option<AssetType>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AssetListResponse {
    pub items: Vec<AssetResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateAssetRequest {
    pub asset_type: AssetType,
    pub external_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAssetBiz {
    pub id: String,
    pub asset_type: AssetType,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub image: AssetImage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAssetRepo {
    pub asset_type: AssetType,
    pub external_id: String,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub image: AssetImage,
    pub ext: AssetExt,
}

impl CreateAssetRepo {
    pub fn from_coin_data(coin_data: CoinDataResponse) -> Self {
        let platform_contract_map = match coin_data.platforms {
            Some(v) => v,
            None => HashMap::new(),
        };
        Self {
            asset_type: AssetType::Crypto,
            external_id: coin_data.id,
            source: "coingecko".to_string(),
            symbol: coin_data.symbol,
            name: coin_data.name,
            image: coin_data.image,
            ext: AssetExt {
                crypto: Some(CryptoExt {
                    platform_contract_map,
                }),
            },
        }
    }
}
