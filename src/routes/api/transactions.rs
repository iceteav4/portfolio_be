use crate::{handlers::transactions::get_transactions, state::AppState};
use axum::{Router, routing::get};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", get(get_transactions))
}
