use std::collections::HashMap;
use std::sync::Arc;

use sqlx::PgPool;

use crate::models::domain::portfolio::CreatePortfolio;
use crate::models::entities::portfolio::Portfolio;
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;

pub struct PortfolioRepo {
    pool: Arc<PgPool>,
}

impl PortfolioRepo {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
    pub async fn create_portfolio(&self, inp: CreatePortfolio) -> Result<Portfolio, sqlx::Error> {
        let id = SNOWFLAKE_GENERATOR.generate().unwrap();
        let mut portfolio = sqlx::query_as!(
            Portfolio,
            r#"
                INSERT INTO portfolios (id, owner_id, name)
                VALUES ($1, $2, $3)
                RETURNING id, owner_id, name, created_at, updated_at
            "#,
            id,
            inp.owner_id,
            inp.name
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        portfolio.positions = HashMap::new();
        Ok(portfolio)
    }
}
