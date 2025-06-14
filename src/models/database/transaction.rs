use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TransactionRow {
    pub id: i64,
    pub external_id: Option<String>,
    pub portfolio_id: i64,
    pub asset_id: String,
    pub tx_type: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub fees: Decimal,
    pub currency: String,
    pub executed_at: OffsetDateTime,
    pub notes: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
