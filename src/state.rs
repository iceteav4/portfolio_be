use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppStateInner {
    pub pg_pool: PgPool,
    pub secret_key: String,
}

pub type AppState = Arc<AppStateInner>;

impl AppStateInner {
    pub fn new(pg_pool: PgPool, secret_key: String) -> Self {
        Self {
            pg_pool,
            secret_key,
        }
    }
}
