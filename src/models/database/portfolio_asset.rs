use serde::Deserialize;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, FromRow)]
pub struct PortfolioAssetRow {
    pub portfolio_id: i64,
    pub asset_id: String,
    pub created_at: OffsetDateTime,
}
