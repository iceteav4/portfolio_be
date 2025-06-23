// use sqlx::PgPool;

// use crate::{
//     db::repositories::{portfolio_asset::PortfolioAssetRepo, transaction::TransactionRepo},
//     models::domain::{portfolio_asset::PortfolioAsset, transaction::Transaction},
//     utils::error::AppError,
// };

// pub struct PortfolioAssetBiz {
//     pool: PgPool,
// }

// impl PortfolioAssetBiz {
//     pub fn new(pool: PgPool) -> Self {
//         Self { pool }
//     }

//     pub async fn get_all_portfolio_assets(
//         self,
//         portfolio_id: i64,
//     ) -> Result<Vec<PortfolioAsset>, AppError> {
//         let port_asset_repo = PortfolioAssetRepo::new(self.pool.clone());
//         let tx_repo = TransactionRepo::new(self.pool.clone());

//         let mut port_assets: Vec<PortfolioAsset> = Vec::new();

//         let port_asset_rows = port_asset_repo
//             .get_multi_by_portfolio_id(portfolio_id)
//             .await?;
//         for pa_row in port_asset_rows.iter() {
//             let tx_rows = tx_repo
//                 .get_multi_txs_by_portfolio_id_asset_id(portfolio_id, &pa_row.asset_id)
//                 .await?;
//             port_assets.push(PortfolioAsset {
//                 portfolio_id,
//                 asset_id: pa_row.asset_id.clone(),
//                 created_at: pa_row.created_at,
//                 transactions: tx_rows
//                     .into_iter()
//                     .map(|tx| Transaction::from_row(tx))
//                     .collect::<Vec<Transaction>>(),
//             });
//         }
//         Ok(port_assets)
//     }
// }
