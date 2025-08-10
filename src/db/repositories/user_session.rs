use sqlx::PgPool;
use time::OffsetDateTime;

use crate::models::domain::user_session::UserSession;
use crate::models::dto::user_session::CreateUserSession;
use crate::utils::error::AppError;
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;

#[derive(Debug)]
pub struct UserSessionRepo {
    pool: PgPool,
}

impl UserSessionRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user_session(
        &self,
        inp: CreateUserSession,
    ) -> Result<UserSession, AppError> {
        let entity = sqlx::query_as!(
            UserSession,
            r#"
            INSERT INTO user_sessions (session_id, user_id, is_active, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING session_id, user_id, is_active, created_at, expires_at
            "#,
            SNOWFLAKE_GENERATOR.generate().unwrap() as i64,
            inp.user_id,
            true,
            OffsetDateTime::now_utc(),
            inp.expires_at
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(entity)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<UserSession>, AppError> {
        let entity = sqlx::query_as!(
            UserSession,
            r#"
            SELECT session_id, user_id, is_active, created_at, expires_at
            FROM user_sessions
            WHERE session_id = $1
            "#,
            id as i64
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(entity)
    }
}
