use sqlx::PgPool;
use time::OffsetDateTime;

use crate::biz::asset::generate_asset_id;
use crate::models::database::asset::AssetRow;
use crate::models::dto::asset::CreateAssetRepo;
use crate::utils::error::AppError;

pub struct AssetRepo {
    pool: PgPool,
}

impl AssetRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_one(&self, inp: CreateAssetRepo) -> Result<String, AppError> {
        Ok(sqlx::query!(
            r#"
            INSERT INTO assets (id, asset_type, external_id, source, symbol, name, image, ext, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
        "#,
            generate_asset_id(&inp.asset_type, &inp.external_id),
            inp.asset_type.to_string(),
            inp.external_id,
            inp.source,
            inp.symbol,
            inp.name,
            serde_json::to_value(&inp.image)?,
            serde_json::to_value(&inp.ext)?,
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc()
        ).fetch_one(&self.pool).await?.id)
    }

    pub async fn get_one_by_id(&self, asset_id: &String) -> Result<Option<AssetRow>, AppError> {
        Ok(sqlx::query_as!(
            AssetRow,
            r#"
                SELECT id, asset_type, external_id, source, symbol, name, image, ext, created_at, updated_at
                FROM assets
                WHERE id = $1
            "#,
            asset_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn get_multi_by_ids(
        &self,
        asset_ids: &Vec<String>,
    ) -> Result<Vec<AssetRow>, AppError> {
        Ok(sqlx::query_as!(
            AssetRow,
            r#"
                SELECT id, asset_type, external_id, source, symbol, name, image, ext, created_at, updated_at
                FROM assets
                WHERE id = ANY($1)
            "#,
            &asset_ids
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_multi_with_paging(
        &self,
        asset_type: Option<String>,
        page: u32,
        limit: u32,
    ) -> Result<Vec<AssetRow>, AppError> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "SELECT id, asset_type, external_id, source, symbol, name, image, ext, created_at, updated_at FROM assets",
        );

        if let Some(asset_type) = asset_type {
            query_builder
                .push(" WHERE asset_type = ")
                .push_bind(asset_type);
        }

        // Add ORDER BY, LIMIT, and OFFSET
        query_builder
            .push(" ORDER BY id ASC LIMIT ")
            .push_bind(limit as i64);
        query_builder.push(" OFFSET ").push_bind((page - 1) as i64);

        Ok(query_builder
            .build_query_as::<AssetRow>()
            .fetch_all(&self.pool)
            .await?)
    }
}
