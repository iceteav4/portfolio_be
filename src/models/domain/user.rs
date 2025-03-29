use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub hashed_password: Option<String>,
    pub name: Option<String>,
}
