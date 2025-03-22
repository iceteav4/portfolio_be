use crate::{
    handlers::auth::{login_with_password, signup},
    state::AppState,
};
use axum::{Router, routing::post};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/login_with_password", post(login_with_password))
}
