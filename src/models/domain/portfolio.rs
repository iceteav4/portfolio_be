use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePortfolio {
    pub owner_id: i64,
    pub name: String,
}
