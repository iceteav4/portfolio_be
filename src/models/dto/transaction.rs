use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use crate::models::{
    common::currency::Currency,
    database::transaction::TransactionRow,
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponse {
    pub id: i64,
    pub external_id: Option<String>,
    pub portfolio_id: i64,
    pub asset_id: String,
    pub fees: String,
    pub executed_at: OffsetDateTime,
    pub notes: Option<String>,
    pub currency: Currency,
    pub quantity: String,
    pub price: String,
    pub tx_type: TxType,
}

impl TransactionResponse {
    pub fn from_db_row(row: TransactionRow) -> Self {
        Self {
            id: row.id,
            external_id: row.external_id,
            portfolio_id: row.portfolio_id,
            asset_id: row.asset_id,
            fees: row.fees.to_string(),
            executed_at: row.executed_at,
            notes: row.notes,
            currency: row.currency.parse().unwrap(),
            quantity: row.quantity.to_string(),
            price: row.price.to_string(),
            tx_type: row.tx_type.parse().unwrap(),
        }
    }
}
