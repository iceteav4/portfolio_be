use serde::{Deserialize, Serialize};
use sqlx::Type;
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Type, PartialEq, ToSchema)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Pending,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Suspended => "suspended",
            UserStatus::Pending => "pending",
        }
    }
}

impl From<String> for UserStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "active" => UserStatus::Active,
            "inactive" => UserStatus::Inactive,
            "suspended" => UserStatus::Suspended,
            "pending" => UserStatus::Pending,
            _ => UserStatus::Inactive,
        }
    }
}

impl ToString for UserStatus {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
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
