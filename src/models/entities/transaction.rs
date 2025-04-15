use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use strum::{Display, EnumString};
use time::OffsetDateTime;

use crate::models::common::currency::Currency;

#[derive(Debug, Serialize, Deserialize, PartialEq, Type, EnumString, Display)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum TxType {
    Buy,
    Sell,
    TransferIn,
    TransferOut,
}

impl From<String> for TxType {
    fn from(s: String) -> Self {
        TxType::from_str(&s).expect("Invalid TxType")
    }
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
