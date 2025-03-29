use std::sync::Arc;

use sqlx::PgPool;

use crate::models::domain::user::CreateUser;
use crate::models::entities::user::User;
use crate::models::entities::user::UserStatus;
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;

#[derive(Debug)]
pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, status, email, hashed_password, name)
            VALUES ($1, $2::public.user_status, $3, $4, $5)
            RETURNING id, email, phone_number, hashed_password, name, status as "status!: UserStatus", created_at, updated_at
            "#,
            SNOWFLAKE_GENERATOR.generate().unwrap() as i64,
            UserStatus::Active as _,
            user.email,
            user.hashed_password,
            user.name,
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, phone_number, hashed_password, name, status as "status!: UserStatus", created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id as i64
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, phone_number, hashed_password, name, status as "status!: UserStatus", created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
