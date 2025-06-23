use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    models::{common::currency::Currency, database::transaction::TransactionRow},
    utils::error::AppError,
};

use super::coingecko::RawTransaction;

#[derive(Debug, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum TxType {
    Buy,
    Sell,
    TransferIn,
    TransferOut,
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
    pub fn from_raw_tx(raw_tx: RawTransaction) -> Result<Self, AppError> {
        Ok(Self {
            external_id: Some(raw_tx.id.to_string()),
            fees: raw_tx.fees.parse()?,
            executed_at: OffsetDateTime::parse(&raw_tx.transaction_timestamp, &Rfc3339)?,
            notes: raw_tx.notes,
            currency: raw_tx.currency.to_lowercase().parse()?,
            quantity: raw_tx.quantity.parse()?,
            price: raw_tx.price.parse()?,
            tx_type: raw_tx.transaction_type.to_lowercase().parse()?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub portfolio_id: i64,
    pub asset_id: String,
    pub tx_type: TxType,
    pub quantity: Decimal,
    pub price: Decimal,
    pub fees: Decimal,
    pub currency: Currency,
    pub executed_at: OffsetDateTime,
    pub notes: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Transaction {
    #[allow(unused)]
    pub fn from_row(row: TransactionRow) -> Self {
        Transaction {
            id: row.id,
            portfolio_id: row.portfolio_id,
            asset_id: row.asset_id.clone(),
            tx_type: row.tx_type.parse().unwrap(),
            quantity: row.quantity,
            price: row.price,
            fees: row.fees,
            currency: row.currency.parse().unwrap(),
            executed_at: row.executed_at,
            notes: row.notes.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
