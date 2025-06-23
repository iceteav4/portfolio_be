use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct AssetRow {
    pub id: String,
    pub asset_type: String,
    pub external_id: String,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub image: serde_json::Value,
    pub ext: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
