use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use sqlx::Type;
#[derive(Debug, Serialize, Deserialize, Type, PartialEq, ToSchema)]
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
