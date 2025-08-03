use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{
    common::asset::{AssetImage, AssetType},
    database::{asset::AssetRow, portfolio::PortfolioRow, transaction::TransactionRow},
};

use super::transaction::TransactionResponse;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePortfolioRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePortfolioAssetRequest {
    pub asset_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioAssetResponse {
    pub id: String,
    pub asset_type: AssetType,
    pub symbol: String,
    pub name: String,
    pub image: AssetImage,
    pub transactions: Vec<TransactionResponse>,
}

impl PortfolioAssetResponse {
    pub fn from_db_rows(asset_row: AssetRow, tx_rows: Vec<TransactionRow>) -> Self {
        Self {
            id: asset_row.id.to_string(),
            asset_type: asset_row.asset_type.parse().unwrap(),
            symbol: asset_row.symbol,
            name: asset_row.name,
            image: serde_json::from_value(asset_row.image).unwrap(),
            transactions: tx_rows
                .into_iter()
                .map(TransactionResponse::from_db_row)
                .collect(),
        }
    }
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
