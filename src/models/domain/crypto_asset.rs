use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::common::asset::AssetImage;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCryptoAsset {
    pub id: String,
    pub source: String,
    pub symbol: String,
    pub name: String,
    pub platform_contract_map: Option<HashMap<String, String>>,
    pub image: AssetImage,
}
