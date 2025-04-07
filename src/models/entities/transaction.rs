use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use time::OffsetDateTime;

use crate::models::common::currency::Currency;

#[derive(Debug, Serialize, Deserialize, PartialEq, Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum TxType {
    Buy,
    Sell,
    TransferIn,
    TransferOut,
}

impl TxType {
    pub fn as_str(&self) -> &str {
        match self {
            TxType::Buy => "buy",
            TxType::Sell => "sell",
            TxType::TransferIn => "transfer_in",
            TxType::TransferOut => "transfer_out",
        }
    }

    pub fn from_str(s: &str) -> Option<TxType> {
        match s {
            "buy" => Some(TxType::Buy),
            "sell" => Some(TxType::Sell),
            "transfer_in" => Some(TxType::TransferIn),
            "transfer_out" => Some(TxType::TransferOut),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub portfolio_id: i64,
    pub asset_id: i64,
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
