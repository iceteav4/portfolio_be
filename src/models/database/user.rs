use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRow {
    pub id: i64,
    pub status: String,
    pub email: String,
    pub phone_number: Option<String>,
    #[serde(skip_serializing)]
    pub hashed_password: Option<String>,
    pub name: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
