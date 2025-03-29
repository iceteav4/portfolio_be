use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct CoinImage {
    pub thumb: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
}

pub struct Coin {
    id: String,
    source: String,
    symbol: String,
    name: String,
    platform_contract_map: Option<HashMap<String, String>>,
    image: CoinImage,
}
