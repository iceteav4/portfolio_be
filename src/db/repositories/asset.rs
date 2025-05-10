use sqlx::PgPool;

use crate::models::database::asset::AssetRow;
use crate::utils::error::AppError;

pub struct AssetRepo {
    pool: PgPool,
}

impl AssetRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn get_multi_by_ids(
        &self,
        asset_ids: &Vec<String>,
    ) -> Result<Vec<AssetRow>, AppError> {
        Ok(sqlx::query_as!(
            AssetRow,
            r#"
                SELECT id, created_at, asset_type, source, symbol, name, image
                FROM assets
                WHERE id = ANY($1)
            "#,
            &asset_ids
        )
        .fetch_all(&self.pool)
        .await?)
    }
}
