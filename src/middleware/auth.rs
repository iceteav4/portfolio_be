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
use serde::{Deserialize, Serialize};

use crate::models::auth::Claims;
use crate::{db::repositories::user::UserRepository, models::user::UserStatus, state::AppState};
use crate::{
    db::repositories::user_session::UserSessionRepository, models::user_session::CreateUserSession,
    utils::error::AppError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CachedClaims {
    claims: Claims,
    is_active: bool,
}

// Convert UserSession to JWT token
pub async fn create_token(
    session: CreateUserSession,
    state: &AppState,
) -> Result<String, AppError> {
    let repository = UserSessionRepository::new(Arc::new(state.pg_pool.clone()));
    let new_session = repository.create_user_session(session).await?;

    // Ensure the claims are properly formatted
    let claims = Claims {
        session_id: new_session.session_id,
        user_id: new_session.user_id,
        exp: new_session.expires_at.unix_timestamp() as usize,
        iat: new_session.created_at.unix_timestamp() as usize,
    };

    encode(&Header::default(), &claims, &state.encoding_key()).map_err(AppError::JwtError)
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

        // Try to get claims from cache first
        let token = bearer.token();
        let cache_key = format!("token:{}", token);

        // Get a Redis connection and check cache
        let mut redis_conn = state.redis_conn.clone();

        if let Ok(cached_result) = redis_conn.get::<_, String>(&cache_key).await {
            if let Ok(cached_claims) = serde_json::from_str::<CachedClaims>(&cached_result) {
                return Ok(cached_claims.claims);
            }
        }

        // If not in cache, validate token and check user status
        let mut validation = Validation::default();
        validation.validate_aud = false;

        let token_data = decode::<Claims>(token, &state.decoding_key(), &validation)
            .map_err(|e| AppError::JwtError(e))?;

        let user_repo = UserRepository::new(Arc::new(state.pg_pool.clone()));
        let is_active = match user_repo.get_by_id(token_data.claims.user_id).await {
            Ok(Some(user)) if user.status == UserStatus::Active => true,
            _ => {
                return Err(AppError::Unauthorized(
                    "User not found or not active".to_string(),
                ));
            }
        };

        // Cache the validated claims
        let cached_claims = CachedClaims {
            claims: token_data.claims.clone(),
            is_active,
        };

        // Cache the validated claims using the connection
        if let Ok(json) = serde_json::to_string(&cached_claims) {
            let _: Result<(), _> = redis_conn
                .set_ex(&cache_key, json, 300) // 300 seconds = 5 minutes
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
