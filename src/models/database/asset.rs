use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

use crate::models::common::asset::{Asset, AssetImage, AssetType};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct AssetRow {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub asset_type: String,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub image: serde_json::Value,
}

impl Asset for AssetRow {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    fn asset_type(&self) -> AssetType {
        self.asset_type.parse().unwrap()
    }

    fn source(&self) -> String {
        self.source.clone()
    }

    fn symbol(&self) -> String {
        self.symbol.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn image(&self) -> AssetImage {
        serde_json::from_value(self.image.clone()).unwrap()
    }
}
