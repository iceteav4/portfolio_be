use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub session_id: i64,
    pub user_id: i64,
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Optional. Issued at (as UTC timestamp)
}

#[derive(Deserialize, ToSchema)]
pub struct LoginWithPasswordRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct SignUpWithPasswordRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub user_id: i64,
    pub token: String,
}
