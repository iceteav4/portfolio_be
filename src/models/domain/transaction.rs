use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::models::entities::{currency::Currency, transaction::TxType};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransaction {
    pub portfolio_id: i64,
    pub asset_id: i64,
    pub fees: Option<Decimal>,
    pub executed_at: OffsetDateTime,
    pub notes: Option<String>,
    pub currency: Currency,
    pub quantity: Decimal,
    pub price: Decimal,
    pub tx_type: TxType,
}
