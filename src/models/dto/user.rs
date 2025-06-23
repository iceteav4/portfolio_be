use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use crate::{models::domain::user::UserStatus, utils::datetime::serialize_datetime};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub hashed_password: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserMeResponse {
    pub id: String,
    pub status: UserStatus,
    pub email: String,
    pub phone_number: Option<String>,
    pub name: Option<String>,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: OffsetDateTime,
}
