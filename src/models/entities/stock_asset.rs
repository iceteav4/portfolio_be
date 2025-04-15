use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::models::common::asset::{Asset, AssetImage, AssetType};

#[derive(Debug, Deserialize, Serialize)]
pub struct StockAsset {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub asset_type: AssetType,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub image: Option<AssetImage>,
}

impl Asset for StockAsset {
    fn id(&self) -> &str {
        &self.id
    }

    fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    fn asset_type(&self) -> AssetType {
        AssetType::Crypto
    }

    fn source(&self) -> &str {
        &self.source
    }

    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn image(&self) -> Option<&AssetImage> {
        self.image.as_ref()
    }
}
