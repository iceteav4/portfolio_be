use sqlx::PgPool;

use crate::models::database::portfolio_asset::PortfolioAssetRow;
use crate::models::domain::portfolio_asset::CreatePortfolioAsset;
use crate::utils::error::AppError;

pub struct PortfolioAssetRepo {
    pool: PgPool,
}

impl PortfolioAssetRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, inp: CreatePortfolioAsset) -> Result<PortfolioAssetRow, AppError> {
        Ok(sqlx::query_as!(
            PortfolioAssetRow,
            r#"
                INSERT INTO portfolio_assets (portfolio_id, asset_id)
                VALUES ($1, $2)
                RETURNING portfolio_id, asset_id, created_at
            "#,
            inp.portfolio_id,
            inp.asset_id,
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
