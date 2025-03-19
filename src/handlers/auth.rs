use std::sync::Arc;

use axum::{Json, Router, extract::State};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{middleware::auth::create_token, models::user_session::UserSession, state::AppState, utils::error::AppError};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(credentials): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Validate credentials (implement your own logic here)
    // Create a new user session
    let session = UserSession {
        session_id: 1, // Generate this
        user_id: 1,    // Get this from your user lookup
        created_at: OffsetDateTime::now_utc(),
        expires_at: None,
    };

    // Create JWT token
    let token = create_token(&session, &state.secret_key)?;

    Ok(Json(LoginResponse { token }))
}
