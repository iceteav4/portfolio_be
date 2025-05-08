use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};
use time::OffsetDateTime;

use crate::models::{common::currency::Currency, database::transaction::TransactionRow};

#[derive(Debug, Serialize, Deserialize, PartialEq, Type, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum TxType {
    Buy,
    Sell,
    TransferIn,
    TransferOut,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
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
    pub fn from_row(row: &TransactionRow) -> Self {
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
