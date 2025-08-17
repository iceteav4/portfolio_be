use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::{IntoParams, ToSchema};

use crate::models::{
    common::currency::Currency,
    database::transaction::TransactionRow,
    domain::transaction::{BaseTransactionInfo, TxType},
};
use crate::utils::datetime::{deserialize_optional_datetime, serialize_datetime};

use super::pagination::{NumberPaginationResponse, default_limit, default_page};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMultiTransaction {
    pub portfolio_id: i64,
    pub asset_id: String,
    pub transactions: Vec<BaseTransactionInfo>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTransactionRequest {
    pub portfolio_id: String,
    pub asset_id: String,
    pub tx_type: TxType,
    pub price: String,
    pub quantity: String,
    pub fees: Option<String>,
    pub currency: Currency,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub executed_at: Option<OffsetDateTime>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTransactionRequest {
    pub tx_type: Option<TxType>,
    pub price: Option<String>,
    pub quantity: Option<String>,
    pub fees: Option<String>,
    pub currency: Option<Currency>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub executed_at: Option<OffsetDateTime>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTransaction {
    pub tx_type: Option<TxType>,
    pub price: Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub fees: Option<Decimal>,
    pub currency: Option<Currency>,
    pub executed_at: Option<OffsetDateTime>,
    pub notes: Option<String>,
}
impl UpdateTransaction {
    pub fn from_req(req: UpdateTransactionRequest) -> Self {
        Self {
            tx_type: req.tx_type,
            price: req.price.map(|p| p.parse().unwrap()),
            quantity: req.quantity.map(|q| q.parse().unwrap()),
            fees: req.fees.map(|f| f.parse().unwrap()),
            currency: req.currency,
            executed_at: req.executed_at,
            notes: req.notes,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionResponse {
    pub id: String,
    pub external_id: Option<String>,
    pub portfolio_id: String,
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
impl TransactionResponse {
    pub fn from_db_row(row: TransactionRow) -> Self {
        Self {
            id: row.id.to_string(),
            external_id: row.external_id,
            portfolio_id: row.portfolio_id.to_string(),
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
    pub number_pagination: NumberPaginationResponse,
    pub items: Vec<TransactionResponse>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct TransactionQueryParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    pub portfolio_id: String,
    pub asset_id: String,
}
