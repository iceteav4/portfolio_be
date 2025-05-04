use super::asset_position::AssetPosition;
use crate::models::database::portfolio::PortfolioRow;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub struct Portfolio {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub positions: HashMap<String, AssetPosition>, // key is asset.id()
}

impl Portfolio {
    pub fn from_row(row: Option<PortfolioRow>) -> Option<Self> {
        row.map(|row| Self {
            id: row.id,
            owner_id: row.owner_id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
            positions: HashMap::new(),
        })
    }

    pub fn total_value(&self) -> Decimal {
        self.positions
            .values()
            .map(|pos| pos.total_buy_value() - pos.total_sell_value())
            .sum()
    }
}
