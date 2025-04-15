use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use strum::{Display, EnumString};
use time::OffsetDateTime;

use crate::{
    models::{
        common::asset::{Asset, AssetImage, AssetType},
        database::{asset::AssetRow, crypto_asset::CryptoAssetRow},
    },
    utils::error::AppError,
};

#[derive(Debug, EnumString, Display, Deserialize, Serialize)]
pub enum CryptoSource {
    CoinGecko,
}

impl From<String> for CryptoSource {
    fn from(value: String) -> Self {
        CryptoSource::from_str(&value).expect("Invalid string for CryptoSource")
    }
}

impl AsRef<str> for CryptoSource {
    fn as_ref(&self) -> &str {
        match self {
            CryptoSource::CoinGecko => "coingecko",
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CryptoAsset {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub asset_type: AssetType,
    pub source: CryptoSource,
    pub symbol: String,
    pub name: String,
    pub image: Option<AssetImage>,
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

    fn image(&self) -> Option<&AssetImage> {
        self.image.as_ref()
    }
}

impl CryptoAsset {
    pub fn from_row(
        asset_row: AssetRow,
        crypto_row: Option<CryptoAssetRow>,
    ) -> Result<Self, AppError> {
        let image: Option<AssetImage> = match asset_row.image {
            Some(v) => Some(serde_json::from_str::<AssetImage>(&v)?),
            None => None,
        };

        let platform_contract_map: HashMap<String, String> = match crypto_row {
            Some(v) => v.platform_contract_map,
            None => HashMap::new(),
        };

        Ok(Self {
            id: asset_row.id,
            created_at: asset_row.created_at,
            asset_type: AssetType::from(asset_row.asset_type),
            source: CryptoSource::from(asset_row.source),
            symbol: asset_row.symbol,
            name: asset_row.name,
            image,
            platform_contract_map,
        })
    }
}
