use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub session_id: i64,
    pub user_id: i64,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
}
