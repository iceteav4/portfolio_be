use super::portfolio_asset::PortfolioAsset;
use crate::models::database::portfolio::PortfolioRow;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub struct Portfolio {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub assets: Vec<PortfolioAsset>,
}

impl Portfolio {
    pub fn from_row(row: Option<PortfolioRow>) -> Option<Self> {
        row.map(|row| Self {
            id: row.id,
            owner_id: row.owner_id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
            assets: Vec::new(),
        })
    }

    pub fn current_balance(self) -> Decimal {
        Decimal::MIN
    }
}
