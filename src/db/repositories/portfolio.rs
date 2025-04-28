use sqlx::PgPool;

use crate::models::database::portfolio::PortfolioRow;
use crate::models::domain::portfolio::CreatePortfolio;
use crate::models::entities::portfolio::Portfolio;
use crate::utils::error::AppError;
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;

pub struct PortfolioRepo {
    pool: PgPool,
}

impl PortfolioRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn create_portfolio(&self, inp: CreatePortfolio) -> Result<Portfolio, AppError> {
        let row = sqlx::query_as!(
            PortfolioRow,
            r#"
                INSERT INTO portfolios (id, owner_id, name)
                VALUES ($1, $2, $3)
                RETURNING id, owner_id, name, created_at, updated_at
            "#,
            SNOWFLAKE_GENERATOR.generate().unwrap(),
            inp.owner_id,
            inp.name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Portfolio::new(row))
    }
}
