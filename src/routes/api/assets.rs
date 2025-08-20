use crate::{
    handlers::assets::{create_asset, get_all_assets, get_detail_asset},
    state::AppState,
};
use axum::{Router, routing::get};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_assets).post(create_asset))
        .route("/{id}", get(get_detail_asset))
}
