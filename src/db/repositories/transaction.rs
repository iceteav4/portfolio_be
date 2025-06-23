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

    // pub async fn create_tx(&self, inp: CreateTransaction) -> Result<TransactionRow, AppError> {
    //     Ok (sqlx::query_as!(TransactionRow,
    //         r#"INSERT INTO transactions (id, external_id, portfolio_id, asset_id, tx_type, quantity, price, fees, currency, executed_at, notes)
    //         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
    //         RETURNING id, external_id, portfolio_id, asset_id, tx_type, quantity, price, fees, currency, executed_at, notes, created_at, updated_at
    //         "#,
    //         SNOWFLAKE_GENERATOR.generate().unwrap(),
    //         inp.external_id,
    //         inp.portfolio_id,
    //         inp.asset_id,
    //         inp.tx_type.to_string(),
    //         inp.quantity,
    //         inp.price,
    //         inp.fees,
    //         inp.currency.as_ref(),
    //         inp.executed_at,
    //         inp.notes,
    //     )
    //     .fetch_one(&self.pool)
    //     .await?)
    // }

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
            b.push_bind(SNOWFLAKE_GENERATOR.generate().unwrap())
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

    pub async fn get_multi_txs_by_portfolio_id_asset_id(
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

    pub async fn update_tx_by_id(
        &self,
        tx_id: i64,
        inp: UpdateTransaction,
    ) -> Result<(), AppError> {
        let mut query_builder = sqlx::QueryBuilder::new("UPDATE transactions SET ");

        let mut separated = query_builder.separated(", ");
        let mut need_update = false;

        if let Some(tx_type) = inp.tx_type {
            separated.push("tx_type = ").push_bind(tx_type.to_string());
            need_update = true;
        }

        if let Some(ref notes) = inp.notes {
            separated.push("notes = ").push_bind(notes);
            need_update = true;
        }

        if let Some(quantity) = inp.quantity {
            separated.push("quantity = ").push_bind(quantity);
            need_update = true;
        }

        if let Some(price) = inp.price {
            separated.push("price = ").push_bind(price);
            need_update = true;
        }

        if let Some(fees) = inp.fees {
            separated.push("fees = ").push_bind(fees);
            need_update = true;
        }

        if let Some(executed_at) = inp.executed_at {
            separated.push("executed_at = ").push_bind(executed_at);
            need_update = true;
        }

        if !need_update {
            // No fields to update
            return Ok(());
        }

        separated
            .push("updated_at = ")
            .push_bind(OffsetDateTime::now_utc());

        query_builder.push(" WHERE id = ").push_bind(tx_id);

        query_builder.build().execute(&self.pool).await?;
        Ok(())
    }
}
