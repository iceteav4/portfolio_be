use serde::Serialize;
use utoipa::ToSchema;

use crate::models::common::asset::{Asset, AssetImage, AssetType};

#[derive(Debug, Serialize, ToSchema)]
pub struct AssetResponse {
    pub id: String,
    pub asset_type: AssetType,
    pub symbol: String,
    pub name: String,
    pub image: AssetImage,
}

impl AssetResponse {
    pub fn from_asset(asset: impl Asset) -> Self {
        Self {
            id: asset.id().to_string(),
            asset_type: asset.asset_type(),
            symbol: asset.symbol().to_string(),
            name: asset.name().to_string(),
            image: asset.image().clone(),
        }
    }
}
