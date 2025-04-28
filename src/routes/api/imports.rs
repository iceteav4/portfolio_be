pub mod coingecko;

use crate::state::AppState;
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/coingecko", coingecko::create_router())
}
