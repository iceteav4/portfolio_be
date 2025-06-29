use crate::{
    handlers::users::{get_user_by_id, get_user_me},
    state::AppState,
};
use axum::{Router, routing::get};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(get_user_by_id))
        .route("/me", get(get_user_me))
}
