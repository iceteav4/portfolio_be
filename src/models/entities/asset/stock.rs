use serde::{Deserialize, Serialize};

use super::{Asset, AssetImage, AssetType};

#[derive(Debug, Deserialize, Serialize)]
pub struct StockAsset {
    pub id: String,
    pub asset_type: AssetType,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub image: AssetImage,
}

impl Asset for StockAsset {
    fn id(&self) -> &str {
        &self.id
    }

    fn asset_type(&self) -> AssetType {
        AssetType::Cryptocurrency
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

    fn image(&self) -> &AssetImage {
        &self.image
    }
}
