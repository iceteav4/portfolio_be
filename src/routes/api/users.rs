use crate::{
    db::repositories::user::UserRepository,
    handlers::users::{create_user, get_user},
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_user))
        .route("/{id}", get(get_user))
}
