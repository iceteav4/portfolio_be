use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct PortfolioAssetRow {
    pub portfolio_id: i64,
    pub asset_id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub holding_amount: Decimal,
    pub total_cost: Decimal,
    pub avg_buy_price: Decimal,
    pub sold_amount: Decimal,
    pub total_revenue: Decimal,
    pub avg_sell_price: Decimal,
}
