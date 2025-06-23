use serde::{Deserialize, Serialize};

use crate::models::common::asset::{AssetExt, AssetImage, AssetType};

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub id: String,
    pub asset_type: AssetType,
    pub external_id: String,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub image: AssetImage,
    pub ext: AssetExt,
}
