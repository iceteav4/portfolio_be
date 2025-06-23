use crate::{
    handlers::portfolios::{
        create_portfolio, create_portfolio_asset, get_my_portfolios, get_portfolio_by_id,
    },
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_portfolio).get(get_my_portfolios))
        .route("/{id}/assets", post(create_portfolio_asset))
        .route("/{id}", get(get_portfolio_by_id))
}
