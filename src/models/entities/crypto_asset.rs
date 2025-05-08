use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{AsRefStr, Display, EnumString};
use time::OffsetDateTime;

use crate::{
    models::{
        common::asset::{Asset, AssetImage, AssetType},
        database::{asset::AssetRow, crypto_asset::CryptoAssetRow},
    },
    utils::error::AppError,
};

#[derive(Debug, EnumString, Display, Deserialize, Serialize, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum CryptoSource {
    CoinGecko,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CryptoAsset {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub asset_type: AssetType,
    pub source: CryptoSource,
    pub symbol: String,
    pub name: String,
    pub image: AssetImage,
    pub external_id: String,
    pub platform_contract_map: HashMap<String, String>,
}

impl Asset for CryptoAsset {
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
        self.source.as_ref()
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

impl CryptoAsset {
    pub fn from_row(asset_row: AssetRow, crypto_row: CryptoAssetRow) -> Result<Self, AppError> {
        let platform_contract_map = serde_json::from_value(crypto_row.platform_contract_map)?;
        Ok(Self {
            id: asset_row.id,
            created_at: asset_row.created_at,
            asset_type: AssetType::from(asset_row.asset_type),
            source: asset_row.source.parse().unwrap(),
            symbol: asset_row.symbol,
            name: asset_row.name,
            image: serde_json::from_value(asset_row.image)?,
            external_id: crypto_row.external_id,
            platform_contract_map,
        })
    }
}
