use crate::state::AppState;
use axum::Router;
pub mod transactions;
pub mod users;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/users", users::create_router())
}
