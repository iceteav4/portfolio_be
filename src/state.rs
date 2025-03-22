use std::sync::Arc;

use jsonwebtoken::{DecodingKey, EncodingKey};
use redis::{Client, aio::ConnectionManager};
use sqlx::PgPool;

#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Database shutdown error: {0}")]
    DatabaseShutdown(String),
    // Add other error types as needed
}

#[derive(Clone)]
pub struct AppStateInner {
    pub pg_pool: PgPool,
    pub secret_key: String,
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
            pg_pool,
            secret_key,
            redis_conn,
        })
    }

    pub fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret_key.as_bytes())
    }

    pub fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret_key.as_bytes())
    }

    pub async fn shutdown(&self) -> Result<(), StateError> {
        // Close database connections
        self.pg_pool.close().await;

        // Redis connection manager will be dropped automatically
        // when it goes out of scope - no explicit drop needed

        Ok(())
    }
}
