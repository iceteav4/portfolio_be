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
    pub image: AssetImage,
}

impl Asset for StockAsset {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    fn asset_type(&self) -> AssetType {
        AssetType::Crypto
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
        self.image.clone()
    }
}
