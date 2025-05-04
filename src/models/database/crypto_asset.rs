use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CryptoAssetRow {
    pub asset_id: String,
    pub external_id: String,
    pub platform_contract_map: serde_json::Value,
}
