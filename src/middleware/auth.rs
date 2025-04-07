use std::sync::Arc;

use axum::{
    RequestPartsExt,
    body::Body,
    extract::{FromRequestParts, Request, State},
    http::StatusCode,
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{Header, Validation, decode, encode};
use redis::AsyncCommands;
use sqlx::PgPool;
use time::OffsetDateTime;

use crate::models::domain::auth::Claims;
use crate::state::AppState;
use crate::{
    db::repositories::user_session::UserSessionRepo,
    models::domain::user_session::CreateUserSession, utils::error::AppError,
};

// Convert UserSession to JWT token
pub async fn create_token(
    session: CreateUserSession,
    state: &AppState,
) -> Result<String, AppError> {
    let repo = UserSessionRepo::new(Arc::new(state.pool.clone()));
    let new_session = repo.create_user_session(session).await?;

    // Ensure the claims are properly formatted
    let claims = Claims {
        session_id: new_session.session_id,
        user_id: new_session.user_id,
        exp: new_session.expires_at.unix_timestamp() as usize,
        iat: new_session.created_at.unix_timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &state.encoding_key())
        .map_err(|_| AppError::Unauthorized("Error when generating token".to_string()))?;

    // Cache the validated claims using the connection
    let cache_key = format!("token:{}", token);

    // Get a Redis connection and check cache
    let mut redis_conn = state.redis_conn.clone();

    if let Ok(json) = serde_json::to_string(&claims) {
        let _: Result<(), _> = redis_conn
            .set_ex(&cache_key, json, 30 * 60) // 30 minutes
            .await;
    }

    Ok(token)
}
async fn is_session_valid(session_id: i64, pool: Arc<PgPool>) -> Result<bool, AppError> {
    let session_repo = UserSessionRepo::new(pool);
    let session = session_repo.get_by_id(session_id).await?;
    match session {
        Some(session) => {
            if !session.is_active {
                return Err(AppError::Unauthorized("Session not active".to_string()));
            }
            if session.expires_at < OffsetDateTime::now_utc() {
                return Err(AppError::Unauthorized("Session expired".to_string()));
            }
            Ok(true)
        }
        None => Err(AppError::Unauthorized("Session not found".to_string())),
    }
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                AppError::Unauthorized("Missing or invalid authorization header".to_string())
            })?;

        let pool = Arc::new(state.pool.clone());

        // Try to get claims from cache first
        let token = bearer.token();
        let cache_key = format!("token:{}", token);

        // Get a Redis connection and check cache
        let mut redis_conn = state.redis_conn.clone();

        if let Ok(cached_result) = redis_conn.get::<_, String>(&cache_key).await {
            if let Ok(claims) = serde_json::from_str::<Claims>(&cached_result) {
                if is_session_valid(claims.session_id, pool.clone()).await? {
                    return Ok(claims);
                }
            }
        }

        // If not in cache, validate token and check user status
        let mut validation = Validation::default();
        validation.validate_aud = false;
        validation.validate_exp = true;

        let token_data = decode::<Claims>(token, &state.decoding_key(), &validation)
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;
        if !is_session_valid(token_data.claims.session_id, pool.clone()).await? {
            return Err(AppError::Unauthorized("Session not valid".to_string()));
        }

        // Cache the validated claims using the connection
        if let Ok(json) = serde_json::to_string(&token_data.claims) {
            let _: Result<(), _> = redis_conn
                .set_ex(&cache_key, json, 30 * 60) // 30 minutes
                .await;
        }

        Ok(token_data.claims)
    }
}

pub async fn require_authentication(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let mut parts = req.into_parts().0;

    // Try to extract and validate the claims
    match Claims::from_request_parts(&mut parts, &state).await {
        Ok(claims) => {
            // Reconstruct the request and add claims to extensions
            let mut req = Request::from_parts(parts, Body::empty());
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            "Invalid or missing authentication token".to_string(),
        )),
    }
}
