use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::models::{common::currency::Currency, entities::transaction::TxType};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransaction {
    pub portfolio_id: i64,
    pub asset_id: String,
    pub fees: Option<Decimal>,
    pub executed_at: OffsetDateTime,
    pub notes: Option<String>,
    pub currency: Currency,
    pub quantity: Decimal,
    pub price: Decimal,
    pub tx_type: TxType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseTransactionInfo {
    pub fees: Option<Decimal>,
    pub executed_at: OffsetDateTime,
    pub notes: Option<String>,
    pub currency: Currency,
    pub quantity: Decimal,
    pub price: Decimal,
    pub tx_type: TxType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMultiTransaction {
    pub portfolio_id: i64,
    pub asset_id: String,
    pub items: Vec<BaseTransactionInfo>,
}
