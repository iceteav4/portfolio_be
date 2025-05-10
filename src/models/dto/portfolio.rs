use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{common::asset::Asset, entities::portfolio::Portfolio};

use super::asset::AssetResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePortfolioRequest {
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioResponse {
    pub id: String,
    pub name: String,
    pub assets: Vec<AssetResponse>,
}

impl PortfolioResponse {
    pub fn from_entity(entity: Portfolio, assets: Vec<impl Asset>) -> Self {
        Self {
            id: entity.id.to_string(),
            name: entity.name,
            assets: assets
                .into_iter()
                .map(|asset| AssetResponse::from_asset(asset))
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BriefPortfolioResponse {
    pub id: String,
    pub name: String,
}

impl BriefPortfolioResponse {
    pub fn from_entity(entity: Portfolio) -> Self {
        Self {
            id: entity.id.to_string(),
            name: entity.name,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BriefPortfolioListResponse {
    pub items: Vec<BriefPortfolioResponse>,
}
