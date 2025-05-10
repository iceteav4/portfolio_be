use sqlx::PgPool;
use time::OffsetDateTime;

use crate::models::database::portfolio_asset::PortfolioAssetRow;
use crate::utils::error::AppError;

pub struct PortfolioAssetRepo {
    pool: PgPool,
}

impl PortfolioAssetRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        portfolio_id: i64,
        asset_id: &String,
    ) -> Result<PortfolioAssetRow, AppError> {
        Ok(sqlx::query_as!(
            PortfolioAssetRow,
            r#"
                INSERT INTO portfolio_assets (portfolio_id, asset_id, created_at)
                VALUES ($1, $2, $3)
                RETURNING portfolio_id, asset_id, created_at
            "#,
            portfolio_id,
            asset_id,
            OffsetDateTime::now_utc(),
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_multi_by_portfolio_id(
        self,
        portfolio_id: i64,
    ) -> Result<Vec<PortfolioAssetRow>, AppError> {
        Ok(sqlx::query_as!(
            PortfolioAssetRow,
            r#"SELECT * FROM portfolio_assets WHERE portfolio_id = $1"#,
            portfolio_id
        )
        .fetch_all(&self.pool)
        .await?)
    }
}
