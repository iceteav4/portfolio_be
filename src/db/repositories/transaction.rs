use sqlx::PgPool;

use crate::models::domain::transaction::CreateTransaction;
use crate::models::entities::transaction::Transaction;
use crate::utils::error::AppError;
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;

pub struct TransactionRepo {
    pool: PgPool,
}

impl TransactionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_transaction(
        &self,
        inp: CreateTransaction,
    ) -> Result<Transaction, AppError> {
        let entity = sqlx::query_as!(Transaction,
            r#"INSERT INTO transactions (id, portfolio_id, asset_id, tx_type, quantity, price, fees, currency, executed_at, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, portfolio_id, asset_id, tx_type, quantity, price, fees, currency, executed_at, notes, created_at, updated_at
            "#,
            SNOWFLAKE_GENERATOR.generate().unwrap(),
            inp.portfolio_id,
            inp.asset_id,
            inp.tx_type.to_string(),
            inp.quantity,
            inp.price,
            inp.fees,
            inp.currency.as_str(),
            inp.executed_at,
            inp.notes,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(entity)
    }
}
