use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use utoipa::ToSchema;

use crate::{
    models::{common::currency::Currency, dto::transaction::CreateTransactionRequest},
    utils::{error::AppError, snowflake::SNOWFLAKE_GENERATOR},
};

use super::coingecko::RawTransaction;

#[derive(Debug, Serialize, Deserialize, EnumString, Display, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum TxType {
    Buy,
    Sell,
    TransferIn,
    TransferOut,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseTransactionInfo {
    pub id: Option<i64>,
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
            id: None,
            external_id: Some(raw_tx.id.to_string()),
            fees: raw_tx.fees.parse().unwrap_or(Decimal::ZERO),
            executed_at: OffsetDateTime::parse(&raw_tx.transaction_timestamp, &Rfc3339)?,
            notes: raw_tx.notes,
            currency: raw_tx.currency.to_uppercase().parse()?,
            quantity: raw_tx.quantity.parse()?,
            price: raw_tx.price.parse()?,
            tx_type: raw_tx.transaction_type.to_uppercase().parse()?,
        })
    }

    pub fn from_create_tx_req(req: CreateTransactionRequest) -> Result<Self, AppError> {
        Ok(Self {
            id: Some(SNOWFLAKE_GENERATOR.generate().unwrap()),
            external_id: None,
            fees: match req.fees {
                Some(fees) => fees.parse()?,
                None => Decimal::ZERO,
            },
            executed_at: req.executed_at.unwrap_or(OffsetDateTime::now_utc()),
            notes: req.notes,
            currency: req.currency,
            quantity: req.quantity.parse()?,
            price: req.price.parse()?,
            tx_type: req.tx_type,
        })
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Transaction {
//     pub id: i64,
//     pub portfolio_id: i64,
//     pub asset_id: String,
//     pub tx_type: TxType,
//     pub quantity: Decimal,
//     pub price: Decimal,
//     pub fees: Decimal,
//     pub currency: Currency,
//     pub executed_at: OffsetDateTime,
//     pub notes: Option<String>,
//     pub created_at: OffsetDateTime,
//     pub updated_at: OffsetDateTime,
// }

// impl Transaction {
//     pub fn from_row(row: TransactionRow) -> Self {
//         Transaction {
//             id: row.id,
//             portfolio_id: row.portfolio_id,
//             asset_id: row.asset_id.clone(),
//             tx_type: row.tx_type.parse().unwrap(),
//             quantity: row.quantity,
//             price: row.price,
//             fees: row.fees,
//             currency: row.currency.parse().unwrap(),
//             executed_at: row.executed_at,
//             notes: row.notes.clone(),
//             created_at: row.created_at,
//             updated_at: row.updated_at,
//         }
//     }
// }
