use sqlx::PgPool;

use crate::models::database::user::UserRow;
use crate::models::domain::user::CreateUser;
use crate::models::entities::user::User;
use crate::models::entities::user::UserStatus;
use crate::utils::error::AppError;
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;

#[derive(Debug)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, inp: CreateUser) -> Result<User, AppError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            INSERT INTO users (id, status, email, hashed_password, name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, phone_number, hashed_password, name, status, created_at, updated_at
            "#,
            SNOWFLAKE_GENERATOR.generate().unwrap() as i64,
            UserStatus::Active.as_str(),
            inp.email,
            inp.hashed_password,
            inp.name,
        )
        .fetch_one(&self.pool)
        .await?;

        User::from_row(Some(row)).ok_or_else(|| AppError::InternalServerError)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, phone_number, hashed_password, name, status, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id as i64
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(User::from_row(row))
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, phone_number, hashed_password, name, status, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(User::from_row(row))
    }
}
