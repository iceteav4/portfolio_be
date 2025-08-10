use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::database::portfolio::PortfolioRow;

use super::portfolio_asset::PortfolioAssetResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePortfolioRequest {
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioResponse {
    pub id: String,
    pub name: String,
    pub assets: Vec<PortfolioAssetResponse>,
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
