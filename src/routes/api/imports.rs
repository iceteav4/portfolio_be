use crate::{handlers::import::coingecko::get_coin_data_by_id, state::AppState};
use axum::{Router, routing::get};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/coingecko/{id}", get(get_coin_data_by_id))
}
