use crate::state::AppState;
use axum::Router;
use std::sync::Arc;
pub mod transactions;
pub mod users;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new().nest("/users", users::create_router())
}
