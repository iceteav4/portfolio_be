use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::{Display, EnumString};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, EnumString, Display, ToSchema)]
pub enum AssetType {
    Crypto,
    Stock,
}

impl From<String> for AssetType {
    fn from(s: String) -> Self {
        AssetType::from_str(&s).expect("Invalid string for AssetType")
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema, Clone)]
#[sqlx(type_name = "JSONB")]
pub struct AssetImage {
    pub thumb: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
}

pub trait Asset {
    fn id(&self) -> String;
    fn created_at(&self) -> OffsetDateTime;
    fn asset_type(&self) -> AssetType;
    fn source(&self) -> String;
    fn symbol(&self) -> String;
    fn name(&self) -> String;
    fn image(&self) -> AssetImage;
}
