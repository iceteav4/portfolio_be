use sqlx::PgPool;
use time::OffsetDateTime;

use crate::models::database::portfolio_asset::PortfolioAssetRow;
use crate::models::dto::portfolio_asset::UpdatePortfolioAsset;
use crate::utils::error::AppError;

pub struct PortfolioAssetRepo {
    pool: PgPool,
}

impl PortfolioAssetRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, portfolio_id: i64, asset_id: &String) -> Result<(), AppError> {
        sqlx::query!(
            r#"
                INSERT INTO portfolio_assets (portfolio_id, asset_id, created_at, updated_at)
                VALUES ($1, $2, $3, $4)
            "#,
            portfolio_id,
            asset_id,
            OffsetDateTime::now_utc(),
            OffsetDateTime::now_utc()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_one_by_portfolio_id_and_asset_id(
        &self,
        portfolio_id: i64,
        asset_id: &String,
    ) -> Result<Option<PortfolioAssetRow>, AppError> {
        Ok(sqlx::query_as!(
            PortfolioAssetRow,
            r#"
                SELECT
                    portfolio_id,
                    asset_id,
                    created_at,
                    updated_at,
                    holding_amount,
                    total_cost,
                    avg_buy_price,
                    sold_amount,
                    total_revenue,
                    avg_sell_price
                FROM portfolio_assets
                WHERE portfolio_id = $1 AND asset_id = $2
            "#,
            portfolio_id,
            asset_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn get_multi_by_portfolio_id(
        self,
        portfolio_id: i64,
    ) -> Result<Vec<PortfolioAssetRow>, AppError> {
        Ok(sqlx::query_as!(
            PortfolioAssetRow,
            r#"
                SELECT
                    portfolio_id,
                    asset_id,
                    created_at,
                    updated_at,
                    holding_amount,
                    total_cost,
                    avg_buy_price,
                    sold_amount,
                    total_revenue,
                    avg_sell_price
                FROM portfolio_assets
                WHERE portfolio_id = $1
            "#,
            portfolio_id
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn update(&self, inp: &UpdatePortfolioAsset) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE portfolio_assets
            SET updated_at = $1,
                holding_amount = $2,
                total_cost = $3,
                avg_buy_price = $4,
                sold_amount = $5,
                total_revenue = $6,
                avg_sell_price = $7
            WHERE portfolio_id = $8 AND asset_id = $9
        "#,
            OffsetDateTime::now_utc(),
            inp.holding_amount,
            inp.total_cost,
            inp.avg_buy_price,
            inp.sold_amount,
            inp.total_revenue,
            inp.avg_sell_price,
            inp.portfolio_id,
            inp.asset_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
