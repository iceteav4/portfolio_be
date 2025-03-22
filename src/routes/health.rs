use crate::handlers::health::health_check;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", get(health_check))
}
