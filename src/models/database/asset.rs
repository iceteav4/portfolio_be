use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

use crate::models::common::asset::AssetType;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct AssetRow {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub asset_type: AssetType,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub image: Option<String>,
}
