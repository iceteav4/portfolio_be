use std::sync::Arc;

use anyhow::Error;
use jsonwebtoken::{DecodingKey, EncodingKey};
use redis::{AsyncCommands, Client, aio::ConnectionManager};
use sqlx::PgPool;

use crate::{clients::app_client::AppClients, config::Settings, db::postgres::init_pg_pool};

#[derive(Clone)]
pub struct AppStateInner {
    pub pool: PgPool,
    secret_key: String,
    pub redis_conn: ConnectionManager,
    pub clients: AppClients,
}

pub type AppState = Arc<AppStateInner>;

impl AppStateInner {
    pub async fn new(app_settings: &Settings) -> Result<Self, Error> {
        // init pg pool
        let pg_pool = init_pg_pool(&app_settings.postgres.url).await?;
        let redis = Client::open(app_settings.redis.url.clone())?;
        let redis_conn = ConnectionManager::new(redis).await?;
        let clients = AppClients::new(&app_settings.clients);
        Ok(Self {
            pool: pg_pool,
            secret_key: app_settings.server.secret_key.clone(),
            redis_conn,
            clients,
        })
    }

    pub async fn health_check(&self) -> Result<(), Error> {
        self.pool.acquire().await?;
        let mut redis_conn = self.redis_conn.clone();
        redis_conn.ping::<String>().await?;
        Ok(())
    }

    pub fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret_key.as_bytes())
    }

    pub fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret_key.as_bytes())
    }

    pub async fn shutdown(&self) -> Result<(), Error> {
        // Close database connections
        self.pool.close().await;

        // Redis connection manager will be dropped automatically
        // when it goes out of scope - no explicit drop needed

        Ok(())
    }
}
