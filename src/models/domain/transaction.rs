use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    models::{common::currency::Currency, entities::transaction::TxType},
    utils::error::AppError,
};

use super::coingecko::RawTransaction;

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
pub struct BaseTransactionInfo {
    pub external_id: Option<String>,
    pub fees: Decimal,
    pub executed_at: OffsetDateTime,
    pub notes: Option<String>,
    pub currency: Currency,
    pub quantity: Decimal,
    pub price: Decimal,
    pub tx_type: TxType,
}

impl BaseTransactionInfo {
    pub fn from_raw_tx(raw_tx: &RawTransaction) -> Result<Self, AppError> {
        Ok(Self {
            external_id: Some(raw_tx.id.to_string()),
            fees: raw_tx.fees.parse()?,
            executed_at: OffsetDateTime::parse(&raw_tx.transaction_timestamp, &Rfc3339)?,
            notes: raw_tx.notes.clone(),
            currency: raw_tx.currency.to_lowercase().parse()?,
            quantity: raw_tx.quantity.parse()?,
            price: raw_tx.price.parse()?,
            tx_type: raw_tx.transaction_type.to_lowercase().parse()?,
        })
    }
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
