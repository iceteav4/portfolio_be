use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::database::portfolio::PortfolioRow;

use super::asset::AssetResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePortfolioRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePortfolioAssetRequest {
    pub portfolio_id: i64,
    pub asset_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioResponse {
    pub id: String,
    pub name: String,
    pub assets: Vec<AssetResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BriefPortfolioResponse {
    pub id: String,
    pub name: String,
}

impl BriefPortfolioResponse {
    pub fn from_row(row: PortfolioRow) -> Self {
        Self {
            id: row.id.to_string(),
            name: row.name,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BriefPortfolioListResponse {
    pub items: Vec<BriefPortfolioResponse>,
}
