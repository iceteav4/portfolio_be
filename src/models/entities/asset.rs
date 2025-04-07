pub mod crypto;
pub mod stock;

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub enum AssetType {
    Cryptocurrency,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetImage {
    pub thumb: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
}

pub trait Asset {
    fn id(&self) -> &str;
    fn asset_type(&self) -> AssetType;
    fn source(&self) -> &str;
    fn symbol(&self) -> &str;
    fn name(&self) -> &str;
    fn image(&self) -> &AssetImage;
}
