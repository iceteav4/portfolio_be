use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::{
    db::repositories::{portfolio_asset::PortfolioAssetRepo, transaction::TransactionRepo},
    models::{domain::transaction::TxType, dto::portfolio_asset::UpdatePortfolioAsset},
    utils::error::AppError,
};

pub async fn update_portfolio_asset_stat(
    pool: PgPool,
    pfl_id: i64,
    asset_id: &String,
) -> Result<(), AppError> {
    let tx_repo = TransactionRepo::new(pool.clone());
    let tx_rows = tx_repo
        .get_multi_txs_by_portfolio_id_asset_id(pfl_id, asset_id, 10_000)
        .await?;

    let mut holding_amount = Decimal::ZERO;
    let mut total_cost = Decimal::ZERO;
    let mut sold_amount = Decimal::ZERO;
    let mut total_revenue = Decimal::ZERO;
    for row in tx_rows {
        let tx_type: TxType = row.tx_type.parse().unwrap();
        match tx_type {
            TxType::Buy => {
                holding_amount += row.quantity;
                total_cost += row.price * row.quantity;
            }
            TxType::Sell => {
                holding_amount -= row.quantity;
                sold_amount += row.quantity;
                total_revenue += row.price * row.quantity;
            }
            TxType::TransferIn => {
                holding_amount += row.quantity;
            }
            TxType::TransferOut => {
                holding_amount -= row.quantity;
            }
        }
    }
    let avg_buy_price = total_cost / holding_amount;
    let avg_sell_price = total_revenue / holding_amount;
    let update_pa = UpdatePortfolioAsset {
        portfolio_id: pfl_id,
        asset_id: asset_id.clone(),
        holding_amount,
        total_cost,
        avg_buy_price,
        sold_amount,
        total_revenue,
        avg_sell_price,
    };
    let pa_repo = PortfolioAssetRepo::new(pool.clone());
    pa_repo.update(&update_pa).await?;
    Ok(())
}
