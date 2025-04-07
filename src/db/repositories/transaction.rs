use std::sync::Arc;

use sqlx::PgPool;

use crate::models::domain::transaction::CreateTransaction;
use crate::models::entities::currency::Currency;
use crate::models::entities::transaction::{Transaction, TxType};
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;
pub struct TransactionRepo {
    pool: Arc<PgPool>,
}

impl TransactionRepo {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_transaction(
        &self,
        inp: CreateTransaction,
    ) -> Result<Transaction, sqlx::Error> {
        sqlx::query_as!(Transaction,
            r#"INSERT INTO transactions (id, portfolio_id, asset_id, tx_type, quantity, price, fees, currency, executed_at, notes) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, portfolio_id, asset_id, tx_type as "tx_type!: TxType", quantity, price, fees, currency as "currency!: Currency", executed_at, notes, created_at, updated_at
            "#,
            SNOWFLAKE_GENERATOR.generate().unwrap(),
            inp.portfolio_id,
            inp.asset_id,
            inp.tx_type as TxType,
            inp.quantity,
            inp.price,
            inp.fees,
            inp.currency as Currency,
            inp.executed_at,
            inp.notes,
        )
        .fetch_one(self.pool.as_ref())
        .await
    }
}
