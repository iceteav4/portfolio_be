use argon2::{self, password_hash::rand_core};
use axum::{Json, extract::State, http::StatusCode};
use time::{Duration, OffsetDateTime};
use tracing::info;

use crate::{
    db::repositories::user::UserRepo,
    middleware::auth::create_token,
    models::dto::{
        api_response::ApiResponse,
        auth::{AuthResponse, LoginWithPasswordRequest, SignUpWithPasswordRequest},
        user::CreateUser,
        user_session::CreateUserSession,
    },
    state::AppState,
};

const BEARER_TOKEN_EXPIRATION: Duration = Duration::days(365);

fn hash_password(password: String) -> Result<String, anyhow::Error> {
    // Hash the password using Argon2
    let salt = argon2::password_hash::SaltString::generate(&mut rand_core::OsRng);
    let argon2 = argon2::Argon2::default();
    let hashed_password =
        argon2::PasswordHasher::hash_password(&argon2, password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!(e))?;
    Ok(hashed_password.to_string())
}

fn verify_password(password: String, hashed_password: String) -> Result<bool, anyhow::Error> {
    let argon2 = argon2::Argon2::default();
    let parsed_hash =
        argon2::PasswordHash::new(&hashed_password).map_err(|e| anyhow::anyhow!(e))?;
    Ok(
        argon2::PasswordVerifier::verify_password(&argon2, password.as_bytes(), &parsed_hash)
            .is_ok(),
    )
}

#[utoipa::path(
    post,
    path = "/auth/login_with_password",
    request_body = LoginWithPasswordRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login_with_password(
    State(state): State<AppState>,
    Json(req): Json<LoginWithPasswordRequest>,
) -> ApiResponse<AuthResponse> {
    // Validate credentials (implement your own logic here)
    info!("Login with password for email: {}", req.email);
    let user_repo = UserRepo::new(state.pool.clone());
    let user = match user_repo.get_by_email(&req.email).await {
        Ok(u) => u,
        Err(e) => {
            return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
        }
    };
    if user.is_none() {
        return ApiResponse::error(StatusCode::NOT_FOUND, "User not found".to_string());
    }
    let user = user.unwrap();
    match verify_password(req.password, user.hashed_password.unwrap()) {
        Ok(true) => {}
        Ok(false) => {
            return ApiResponse::error(StatusCode::UNAUTHORIZED, "Invalid password".to_string());
        }
        Err(e) => {
            return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
        }
    }
    // Create a new user session
    let create_session = CreateUserSession {
        user_id: user.id,
        expires_at: OffsetDateTime::now_utc() + BEARER_TOKEN_EXPIRATION,
    };

    // Create JWT token
    let token = match create_token(create_session, &state).await {
        Ok(t) => t,
        Err(e) => {
            return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
        }
    };
    return ApiResponse::success(AuthResponse {
        user_id: user.id,
        token: token,
    });
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = SignUpWithPasswordRequest,
    responses(
        (status = 200, description = "Signup successful", body = AuthResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn signup(
    State(state): State<AppState>,
    Json(req): Json<SignUpWithPasswordRequest>,
) -> ApiResponse<AuthResponse> {
    let user_repo = UserRepo::new(state.pool.clone());
    match user_repo.get_by_email(&req.email).await {
        Ok(u) => {
            if u.is_some() {
                return ApiResponse::error(
                    StatusCode::BAD_REQUEST,
                    "User already exists".to_string(),
                );
            }
        }
        Err(e) => {
            return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
        }
    };
    let hashed_password = match hash_password(req.password) {
        Ok(hash) => hash,
        Err(e) => {
            return ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to hash password: {}", e),
            );
        }
    };

    let user = match user_repo
        .create_user(CreateUser {
            email: req.email,
            hashed_password: Some(hashed_password),
            name: Some(req.name),
        })
        .await
    {
        Ok(u) => u,
        Err(e) => {
            return ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create user: {}", e),
            );
        }
    };
    let create_session = CreateUserSession {
        user_id: user.id,
        expires_at: OffsetDateTime::now_utc() + BEARER_TOKEN_EXPIRATION,
    };
    let token = match create_token(create_session, &state).await {
        Ok(t) => t,
        Err(e) => {
            return ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create token: {}", e),
            );
        }
    };
    return ApiResponse::success(AuthResponse {
        user_id: user.id,
        token: token,
    });
}
