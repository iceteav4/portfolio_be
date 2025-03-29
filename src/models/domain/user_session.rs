use time::OffsetDateTime;

// Input type for creating user sessions
#[derive(Debug)]
pub struct CreateUserSession {
    pub user_id: i64,
    pub expires_at: OffsetDateTime,
}
