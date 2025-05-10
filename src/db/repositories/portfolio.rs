use sqlx::PgPool;

use crate::models::database::portfolio::PortfolioRow;
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
    pub async fn create(&self, owner_id: i64, name: &str) -> Result<Portfolio, AppError> {
        let row = sqlx::query_as!(
            PortfolioRow,
            r#"
                INSERT INTO portfolios (id, owner_id, name)
                VALUES ($1, $2, $3)
                RETURNING id, owner_id, name, created_at, updated_at
            "#,
            SNOWFLAKE_GENERATOR.generate().unwrap(),
            owner_id,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Portfolio::from_row(row))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Portfolio>, AppError> {
        let row = sqlx::query_as!(
            PortfolioRow,
            r#"
                SELECT id, owner_id, name, created_at, updated_at
                FROM portfolios
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Portfolio::from_row(row))),
            None => Ok(None),
        }
    }

    pub async fn get_multi_by_owner_id(&self, owner_id: i64) -> Result<Vec<Portfolio>, AppError> {
        let rows = sqlx::query_as!(
            PortfolioRow,
            r#"
                SELECT id, owner_id, name, created_at, updated_at
                FROM portfolios
                WHERE owner_id = $1
            "#,
            owner_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Portfolio::from_row(row))
            .collect::<Vec<Portfolio>>())
    }
}
