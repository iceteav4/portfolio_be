use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::utils::error::AppError;
use crate::{models::user_session::UserSession, state::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub session_id: i64,
    pub user_id: i64,
    pub exp: i64,
}

// Convert UserSession to JWT token
pub fn create_token(session: &UserSession, secret: &str) -> Result<String, AppError> {
    let expires_at = session
        .expires_at
        .unwrap_or_else(|| OffsetDateTime::now_utc().saturating_add(time::Duration::days(7)));

    let claims = Claims {
        session_id: session.session_id,
        user_id: session.user_id,
        exp: expires_at.unix_timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(AppError::JwtError)
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                AppError::Unauthorized("Missing or invalid authorization header".to_string())
            })?;

        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(state.secret_key.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::JwtError(e))?;

        Ok(token_data.claims)
    }
}
