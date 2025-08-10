use rust_decimal::{Decimal, prelude::ToPrimitive};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePortfolioAssetRequest {
    pub asset_id: String,
}

pub struct UpdatePortfolioAsset {
    pub portfolio_id: i64,
    pub asset_id: String,
    pub holding_amount: Decimal,
    pub total_cost: Decimal,
    pub avg_buy_price: Decimal,
    pub sold_amount: Decimal,
    pub total_revenue: Decimal,
    pub avg_sell_price: Decimal,
}

use crate::models::{
    common::asset::{AssetImage, AssetType},
    database::{asset::AssetRow, portfolio_asset::PortfolioAssetRow},
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePortfolioRequest {
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioAssetStat {
    pub holding_amount: f64,
    pub holding_value: f64,
    pub total_cost: f64,
    pub avg_buy_price: f64,
    pub total_revenue: f64,
    pub avg_sell_price: f64,
    pub profit_loss: f64,
}
impl PortfolioAssetStat {
    pub fn from_db_row(pfl_asset_row: &PortfolioAssetRow, current_price: Decimal) -> Self {
        Self {
            holding_amount: pfl_asset_row.holding_amount.to_f64().unwrap(),
            holding_value: (pfl_asset_row.holding_amount * current_price)
                .to_f64()
                .unwrap(),
            total_cost: pfl_asset_row.total_cost.to_f64().unwrap(),
            avg_buy_price: pfl_asset_row.avg_buy_price.to_f64().unwrap(),
            total_revenue: pfl_asset_row.total_revenue.to_f64().unwrap(),
            avg_sell_price: pfl_asset_row.avg_sell_price.to_f64().unwrap(),
            profit_loss: (pfl_asset_row.holding_amount * current_price
                + pfl_asset_row.total_revenue
                - pfl_asset_row.total_cost)
                .to_f64()
                .unwrap(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PortfolioAssetResponse {
    pub id: String,
    pub asset_type: AssetType,
    pub symbol: String,
    pub name: String,
    pub image: AssetImage,
    pub stats: PortfolioAssetStat,
}

impl PortfolioAssetResponse {
    pub fn from_db_row(
        asset_row: &AssetRow,
        pfl_asset_row: &PortfolioAssetRow,
        current_price: Decimal,
    ) -> Self {
        Self {
            id: asset_row.id.to_string(),
            asset_type: asset_row.asset_type.parse().unwrap(),
            symbol: asset_row.symbol.clone(),
            name: asset_row.name.clone(),
            image: serde_json::from_value(asset_row.image.clone()).unwrap(),
            stats: PortfolioAssetStat::from_db_row(pfl_asset_row, current_price),
        }
    }
}
