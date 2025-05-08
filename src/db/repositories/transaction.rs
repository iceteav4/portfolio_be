use sqlx::PgPool;

use crate::models::database::transaction::TransactionRow;
use crate::models::domain::transaction::{CreateMultiTransaction, CreateTransaction};
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
        let row = sqlx::query_as!(TransactionRow,
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
            inp.currency.as_ref(),
            inp.executed_at,
            inp.notes,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Transaction::from_row(&row))
    }

    pub async fn create_multi_transaction(
        &self,
        inp: CreateMultiTransaction,
    ) -> Result<u64, AppError> {
        if inp.transactions.is_empty() {
            return Ok(0);
        }

        let portfolio_id = inp.portfolio_id;
        let asset_id = inp.asset_id.clone();

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO transactions (id, portfolio_id, asset_id, tx_type, quantity, price, fees, currency, executed_at, notes) ",
        );

        query_builder.push_values(inp.transactions, |mut b, item| {
            b.push_bind(SNOWFLAKE_GENERATOR.generate().unwrap())
                .push_bind(portfolio_id)
                .push_bind(&asset_id)
                .push_bind(item.tx_type.to_string())
                .push_bind(item.quantity)
                .push_bind(item.price)
                .push_bind(item.fees)
                .push_bind(item.currency.to_string())
                .push_bind(item.executed_at)
                .push_bind(item.notes);
        });

        let result = query_builder.build().execute(&self.pool).await?;

        Ok(result.rows_affected())
    }

    pub async fn get_multi_transactions_by_portfolio_asset(
        &self,
        portfolio_id: i64,
        asset_id: &str,
    ) -> Result<Vec<TransactionRow>, AppError> {
        Ok(sqlx::query_as!(
            TransactionRow,
            r#"SELECT * FROM transactions WHERE portfolio_id = $1 AND asset_id = $2"#,
            portfolio_id,
            asset_id
        )
        .fetch_all(&self.pool)
        .await?)
    }
}
