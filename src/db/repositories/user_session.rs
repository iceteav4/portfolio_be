use std::sync::Arc;

use crate::models::domain::user_session::CreateUserSession;
use crate::models::entities::user_session::UserSession;
use crate::utils::snowflake::SNOWFLAKE_GENERATOR;
use sqlx::PgPool;
use time::OffsetDateTime;

#[derive(Debug)]
pub struct UserSessionRepository {
    pool: Arc<PgPool>,
}

impl UserSessionRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_user_session(
        &self,
        session: CreateUserSession,
    ) -> Result<UserSession, sqlx::Error> {
        sqlx::query_as!(
            UserSession,
            r#"
            INSERT INTO user_sessions (session_id, user_id, is_active, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING session_id, user_id, is_active, created_at, expires_at
            "#,
            SNOWFLAKE_GENERATOR.generate().unwrap() as i64,
            session.user_id,
            true,
            OffsetDateTime::now_utc(),
            session.expires_at
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<UserSession>, sqlx::Error> {
        sqlx::query_as!(
            UserSession,
            r#"
            SELECT session_id, user_id, is_active, created_at, expires_at
            FROM user_sessions
            WHERE session_id = $1
            "#,
            id as i64
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
