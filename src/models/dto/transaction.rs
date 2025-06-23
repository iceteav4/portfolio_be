use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::models::{
    common::currency::Currency,
    domain::transaction::{BaseTransactionInfo, TxType},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransaction {
    pub external_id: Option<String>,
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
pub struct CreateMultiTransaction {
    pub portfolio_id: i64,
    pub asset_id: String,
    pub transactions: Vec<BaseTransactionInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTransaction {
    pub tx_type: Option<TxType>,
    pub notes: Option<String>,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub fees: Option<Decimal>,
    pub executed_at: Option<OffsetDateTime>,
}
