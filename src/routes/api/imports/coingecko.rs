use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::import::coingecko::{get_coin_data_by_id, import_portfolio_file},
    state::AppState,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_coin_data_by_id))
        .route("/upload_portfolio_file", post(import_portfolio_file))
}
