use crate::{models::user_session::UserSession, state::AppState, utils::error::AppError};
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use jsonwebtoken::{DecodingKey, Validation, decode};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// ... existing code ...

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    session_id: i64,
    exp: i64, // Expiration time
    iat: i64, // Issued at
}

pub async fn generate_token(session_id: i64, secret_key: &str) -> Result<String, AppError> {
    let now = OffsetDateTime::now_utc();
    let claims = Claims {
        session_id,
        exp: (now + time::Duration::hours(24)).unix_timestamp(), // Token expires in 24 hours
        iat: now.unix_timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .map_err(AppError::JwtError)
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for UserSession
where
    S: Send + Sync,
    S: AsRef<AppState>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract bearer token
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::Unauthorized("Invalid or missing token".into()))?;

        // Get secret key from state/settings
        let secret_key = state.as_ref().secret_key.as_bytes();

        // Decode and validate token
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret_key),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

        // Query session from database
        let session = sqlx::query_as!(
            UserSession,
            r#"
            SELECT user_id, created_at, expires_at
            FROM user_sessions
            WHERE id = $1 AND expires_at > NOW()
            "#,
            token_data.claims.session_id
        )
        .fetch_optional(&state.as_ref().pg_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?
        .ok_or_else(|| AppError::Unauthorized("Session not found or expired".into()))?;

        Ok(session)
    }
}

// Helper function to create a new session
pub async fn create_session(user_id: i64, state: &AppState) -> Result<String, AppError> {
    let expires_at = OffsetDateTime::now_utc() + time::Duration::hours(24);

    // Insert new session into database
    let session = sqlx::query!(
        r#"
        INSERT INTO user_sessions (user_id, expires_at)
        VALUES ($1, $2)
        RETURNING id
        "#,
        user_id,
        expires_at,
    )
    .fetch_one(&state.pg_pool)
    .await
    .map_err(AppError::DatabaseError)?;

    // Generate JWT token
    generate_token(session.id, &state.secret_key).await
}
