use std::sync::Arc;

use jsonwebtoken::{DecodingKey, EncodingKey};
use redis::{AsyncCommands, Client, aio::ConnectionManager};
use sqlx::PgPool;

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Database error: {0}")]
    Database(String),
    // Add other error types as needed
}

#[derive(Clone)]
pub struct AppStateInner {
    pub pool: PgPool,
    secret_key: String,
    pub redis_conn: ConnectionManager,
}

pub type AppState = Arc<AppStateInner>;

impl AppStateInner {
    pub async fn new(
        pg_pool: PgPool,
        secret_key: String,
        redis_url: String,
    ) -> Result<Self, StateError> {
        let redis = Client::open(redis_url)?;
        let redis_conn = ConnectionManager::new(redis).await?;
        Ok(Self {
            pool: pg_pool,
            secret_key,
            redis_conn,
        })
    }

    pub async fn health_check(&self) -> Result<(), StateError> {
        self.pool
            .acquire()
            .await
            .map_err(|e| StateError::Database(e.to_string()))?;
        let mut redis_conn = self.redis_conn.clone();
        redis_conn
            .ping::<String>()
            .await
            .map_err(|e| StateError::Redis(e))?;
        Ok(())
    }

    pub fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret_key.as_bytes())
    }

    pub fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret_key.as_bytes())
    }

    pub async fn shutdown(&self) -> Result<(), StateError> {
        // Close database connections
        self.pool.close().await;

        // Redis connection manager will be dropped automatically
        // when it goes out of scope - no explicit drop needed

        Ok(())
    }
}
