use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Asset, AssetImage, AssetType};

#[derive(Debug, Deserialize, Serialize)]
pub struct CryptoAsset {
    pub id: String,
    pub asset_type: AssetType,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub platform_contract_map: Option<HashMap<String, String>>,
    pub image: AssetImage,
}

impl Asset for CryptoAsset {
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
