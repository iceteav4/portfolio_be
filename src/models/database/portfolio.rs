use serde::Deserialize;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct PortfolioRow {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
