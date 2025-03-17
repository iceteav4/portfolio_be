use std::sync::Arc;

use sqlx::PgPool;

use crate::models::user::User;
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;

#[derive(Debug)]
pub struct UserRepository {
    pool: Arc<PgPool>,
}

// Input type for creating users
#[derive(Debug)]
pub struct CreateUser {
    pub name: String,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, name)
            VALUES ($1, $2)
            RETURNING id, email, phone_number, name, created_at, updated_at
            "#,
            SNOWFLAKE_GENERATOR.generate().unwrap() as i64,
            user.name
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
            id as i64
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
