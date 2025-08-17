use sqlx::PgPool;
use time::OffsetDateTime;

use crate::models::database::transaction::TransactionRow;
use crate::models::dto::transaction::{CreateMultiTransaction, UpdateTransaction};
use crate::utils::error::AppError;
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;

pub struct TransactionRepo {
    pool: PgPool,
}

impl TransactionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_multi_txs(&self, inp: CreateMultiTransaction) -> Result<u64, AppError> {
        if inp.transactions.is_empty() {
            return Ok(0);
        }

        let portfolio_id = inp.portfolio_id;
        let asset_id = inp.asset_id.clone();

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO transactions (id, external_id, portfolio_id, asset_id, tx_type, quantity, price, fees, currency, executed_at, notes) ",
        );

        query_builder.push_values(inp.transactions, |mut b, item| {
            b.push_bind(item.id.unwrap_or(SNOWFLAKE_GENERATOR.generate().unwrap()))
                .push_bind(item.external_id)
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

    pub async fn get_multi_txs_by_portfolio_and_asset_with_paging(
        &self,
        portfolio_id: i64,
        asset_id: &str,
        page: u32,
        limit: u32,
    ) -> Result<Vec<TransactionRow>, AppError> {
        let query_limit = limit as i64;
        let query_offset = ((page - 1) * limit) as i64;
        Ok(sqlx::query_as!(
            TransactionRow,
            r#"SELECT * FROM transactions WHERE portfolio_id = $1 AND asset_id = $2 ORDER BY executed_at DESC LIMIT $3 OFFSET $4"#,
            portfolio_id,
            asset_id,
            query_limit,
            query_offset
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_one_by_id(&self, tx_id: i64) -> Result<Option<TransactionRow>, AppError> {
        Ok(sqlx::query_as!(
            TransactionRow,
            r#"SELECT * FROM transactions WHERE id = $1"#,
            tx_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn count_txs_by_portfolio_and_asset(
        &self,
        portfolio_id: i64,
        asset_id: &str,
    ) -> Result<Option<i64>, AppError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM transactions WHERE portfolio_id = $1 AND asset_id = $2"#,
            portfolio_id,
            asset_id
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn update_tx_by_id(
        &self,
        tx_id: i64,
        inp: UpdateTransaction,
    ) -> Result<(), AppError> {
        let mut query_builder = sqlx::QueryBuilder::new("UPDATE transactions SET ");

        let mut separated = query_builder.separated(", ");
        let mut need_update = false;

        if let Some(tx_type) = inp.tx_type {
            separated
                .push("tx_type = ")
                .push_bind_unseparated(tx_type.to_string());
            need_update = true;
        }

        if let Some(ref notes) = inp.notes {
            separated.push("notes = ").push_bind_unseparated(notes);
            need_update = true;
        }

        if let Some(quantity) = inp.quantity {
            separated
                .push("quantity = ")
                .push_bind_unseparated(quantity);
            need_update = true;
        }

        if let Some(price) = inp.price {
            separated.push("price = ").push_bind_unseparated(price);
            need_update = true;
        }

        if let Some(currency) = inp.currency {
            separated
                .push("currency = ")
                .push_bind_unseparated(currency.to_string());
            need_update = true;
        }

        if let Some(fees) = inp.fees {
            separated.push("fees = ").push_bind_unseparated(fees);
            need_update = true;
        }

        if let Some(executed_at) = inp.executed_at {
            separated
                .push("executed_at = ")
                .push_bind_unseparated(executed_at);
            need_update = true;
        }

        if !need_update {
            // No fields to update
            return Ok(());
        }

        separated
            .push("updated_at = ")
            .push_bind_unseparated(OffsetDateTime::now_utc());

        query_builder.push(" WHERE id = ").push_bind(tx_id);

        query_builder.build().execute(&self.pool).await?;
        Ok(())
    }
}
