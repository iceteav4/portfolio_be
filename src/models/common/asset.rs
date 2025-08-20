use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, EnumString, Display, ToSchema, PartialEq)]
#[strum(serialize_all = "UPPERCASE")]
pub enum AssetType {
    #[serde(rename = "CRYPTO")]
    Crypto,
    #[serde(rename = "STOCK")]
    Stock,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema, Clone)]
#[sqlx(type_name = "JSONB")]
pub struct AssetImage {
    pub thumb: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoExt {
    pub platform_contract_map: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetExt {
    pub crypto: Option<CryptoExt>,
}
