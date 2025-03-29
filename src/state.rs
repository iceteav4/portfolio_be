use std::sync::Arc;

use jsonwebtoken::{DecodingKey, EncodingKey};
use redis::{AsyncCommands, Client, aio::ConnectionManager};
use sqlx::PgPool;

use crate::{clients::coingecko::CoinGeckoClient, config::Settings, db::postgres::init_pg_pool};

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
    pub coingecko_client: CoinGeckoClient,
}

pub type AppState = Arc<AppStateInner>;

impl AppStateInner {
    pub async fn new(app_settings: &Settings) -> Result<Self, StateError> {
        // init pg pool
        let pg_pool = init_pg_pool(&app_settings.postgres.url)
            .await
            .map_err(|e| StateError::Database(e.to_string()))?;
        let redis = Client::open(app_settings.redis.url.clone())?;
        let redis_conn = ConnectionManager::new(redis).await?;
        let coingecko_client = CoinGeckoClient::new(app_settings.coingecko.api_key.clone());
        Ok(Self {
            pool: pg_pool,
            secret_key: app_settings.server.secret_key.clone(),
            redis_conn,
            coingecko_client,
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
