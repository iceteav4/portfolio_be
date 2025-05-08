use sqlx::PgPool;

use crate::{models::entities::portfolio::Portfolio, utils::error::AppError};

pub struct PortfolioBiz {
    pool: PgPool,
}

impl PortfolioBiz {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn get_by_portfolio_id(self, portfolio_id: i64) -> Result<Portfolio, AppError> {
        let mut portfolio = sqlx::query_as!(
            Portfolio,
            "SELECT * FROM portfolios WHERE id = $1",
            portfolio_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(portfolio)
    }
}
