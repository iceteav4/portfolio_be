use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::transaction::Transaction;

#[derive(Debug, Deserialize, Serialize)]
pub struct PortfolioAsset {
    pub portfolio_id: i64,
    pub asset_id: String,
    pub created_at: OffsetDateTime,
    pub transactions: Vec<Transaction>,
}

impl PortfolioAsset {
    pub fn holding(self) -> Decimal {
        Decimal::MIN
    }

    pub fn get_pnl_value(self) -> Decimal {
        Decimal::MIN
    }

    pub fn get_pnl_percent(self) -> Decimal {
        Decimal::MIN
    }
}
