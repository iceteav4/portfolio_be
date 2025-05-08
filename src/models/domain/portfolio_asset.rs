use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePortfolioAsset {
    pub portfolio_id: i64,
    pub asset_id: String,
}
