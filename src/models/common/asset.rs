use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::{Display, EnumString};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize, EnumString, Display)]
pub enum AssetType {
    Crypto,
    Stock,
}

impl From<String> for AssetType {
    fn from(s: String) -> Self {
        AssetType::from_str(&s).expect("Invalid string for AssetType")
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[sqlx(type_name = "JSONB")]
pub struct AssetImage {
    pub thumb: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
}

pub trait Asset {
    fn id(&self) -> &str;
    fn created_at(&self) -> OffsetDateTime;
    fn asset_type(&self) -> AssetType;
    fn source(&self) -> &str;
    fn symbol(&self) -> &str;
    fn name(&self) -> &str;
    fn image(&self) -> &AssetImage;
}
