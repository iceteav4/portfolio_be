use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::portfolio_asset::PortfolioAsset;

#[derive(Debug, Deserialize, Serialize)]
pub struct Portfolio {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub assets: Vec<PortfolioAsset>,
}
