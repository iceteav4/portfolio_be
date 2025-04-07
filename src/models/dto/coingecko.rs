use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::entities::asset::AssetImage;

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinDataResponse {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: Option<HashMap<String, String>>,
    pub image: AssetImage,
}
