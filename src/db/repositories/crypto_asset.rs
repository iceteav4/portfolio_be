use sqlx::PgPool;
use time::OffsetDateTime;

use crate::models::common::asset::AssetType;
use crate::models::database::{asset::AssetRow, crypto_asset::CryptoAssetRow};
use crate::models::{
    domain::crypto_asset::CreateCryptoAsset,
    entities::crypto_asset::{CryptoAsset, CryptoSource},
};
use crate::utils::error::AppError;

pub struct CryptoAssetRepo {
    pool: PgPool,
}

impl CryptoAssetRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn create_crypto_asset(
        &self,
        inp: CreateCryptoAsset,
    ) -> Result<CryptoAsset, AppError> {
        let mut tx = self.pool.begin().await?;

        let asset_row = sqlx::query_as!(
            AssetRow,
            r#"
                INSERT INTO assets (id, created_at, asset_type, source, symbol, name, image)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, created_at, asset_type, source, symbol, name, image
            "#,
            inp.id,
            OffsetDateTime::now_utc(),
            AssetType::Crypto.to_string(),
            CryptoSource::CoinGecko.to_string(),
            inp.symbol,
            inp.name,
            serde_json::to_value(inp.image)?
        )
        .fetch_one(&mut *tx)
        .await?;

        // Convert contract map to JSON Value
        let contract_json = serde_json::to_value(&inp.platform_contract_map)?;
        let crypto_asset_row = sqlx::query_as!(
            CryptoAssetRow,
            r#"
                INSERT INTO crypto_assets (asset_id, external_id, platform_contract_map)
                VALUES ($1, $2, $3)
                RETURNING asset_id, external_id, platform_contract_map
            "#,
            asset_row.id,
            inp.external_id,
            contract_json
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        let crypto_asset = CryptoAsset::from_row(asset_row, crypto_asset_row)?;
        Ok(crypto_asset)
    }

    pub async fn get_by_id(&self, asset_id: &String) -> Result<Option<CryptoAsset>, AppError> {
        let asset_row = sqlx::query_as!(
            AssetRow,
            r#"
                SELECT id, created_at, asset_type, source, symbol, name, image
                FROM assets
                WHERE id = $1
            "#,
            asset_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let crypto_asset_row = sqlx::query_as!(
            CryptoAssetRow,
            r#"
                SELECT asset_id, external_id, platform_contract_map
                FROM crypto_assets
                WHERE asset_id = $1
            "#,
            asset_id
        )
        .fetch_optional(&self.pool)
        .await?;

        match (asset_row, crypto_asset_row) {
            (Some(asset_row), Some(crypto_asset_row)) => {
                let crypto_asset = CryptoAsset::from_row(asset_row, crypto_asset_row)?;
                Ok(Some(crypto_asset))
            }
            _ => Ok(None),
        }
    }
}
