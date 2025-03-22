use serde::{Deserialize, Serialize};
use sqlx::Type;
use time::OffsetDateTime;
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "user_status")] // matches the PostgreSQL type name
#[sqlx(rename_all = "lowercase")] // ensures enum variants match DB values
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Pending,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub status: UserStatus,
    pub email: String,
    pub phone_number: Option<String>,
    #[serde(skip_serializing)]
    pub hashed_password: Option<String>,
    pub name: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i64,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub hashed_password: Option<String>,
    pub name: Option<String>,
}
