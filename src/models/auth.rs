use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginWithPasswordRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct SignUpWithPasswordRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    user_id: i64,
    token: String,
}
