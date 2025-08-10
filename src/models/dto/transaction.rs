use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::{IntoParams, ToSchema};

use crate::models::{
    common::currency::Currency,
    database::transaction::TransactionRow,
    domain::transaction::{BaseTransactionInfo, TxType},
};
use crate::utils::datetime::serialize_datetime;

use super::pagination::{CursorPaginationQuery, CursorPaginationResponse};

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
    #[serde(serialize_with = "serialize_datetime")]
    pub executed_at: OffsetDateTime,
    pub notes: Option<String>,
    pub currency: Currency,
    pub quantity: String,
    pub price: String,
    pub tx_type: TxType,
}
#[allow(dead_code)]
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

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionListResponse {
    #[serde(flatten)]
    pub cursor_pagination: CursorPaginationResponse,
    pub items: Vec<TransactionResponse>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct TransactionQueryParams {
    #[serde(flatten)]
    pub pagination: CursorPaginationQuery,
    pub portfolio_id: String,
    pub asset_id: String,
}
