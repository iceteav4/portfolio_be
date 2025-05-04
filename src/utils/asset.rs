use crate::models::common::asset::AssetType;

pub fn get_asset_id(asset_type: AssetType, external_id: &str) -> String {
    match asset_type {
        AssetType::Crypto => format!("crypto_{}", external_id),
        AssetType::Stock => format!("stock_{}", external_id),
    }
}
