// use sqlx::PgPool;

// use crate::{
//     db::repositories::{portfolio::PortfolioRepo, transaction::TransactionRepo},
//     models::domain::{portfolio::Portfolio, transaction::Transaction},
//     utils::error::AppError,
// };

// use super::portfolio_asset::PortfolioAssetBiz;

// pub struct PortfolioBiz {
//     pool: PgPool,
// }

// impl PortfolioBiz {
//     pub fn new(pool: PgPool) -> Self {
//         Self { pool }
//     }

//     pub async fn get_by_portfolio_id(
//         &self,
//         portfolio_id: i64,
//     ) -> Result<Option<Portfolio>, AppError> {
//         let portfolio_repo = PortfolioRepo::new(self.pool.clone());
//         let portfolio_row = portfolio_repo.get_one_by_id(portfolio_id).await?;
//         if portfolio_row.is_none() {
//             return Ok(None);
//         }
//         let transaction_repo = TransactionRepo::new(self.pool.clone());
//         let portfolio_row = portfolio_row.unwrap();
//         let pa_repo = PortfolioAssetBiz::new(self.pool.clone());
//         let mut all_portfolio_assets = pa_repo.get_all_portfolio_assets(portfolio_id).await?;
//         for pa in all_portfolio_assets.iter_mut() {
//             pa.portfolio_id = portfolio_id;
//             let tx_rows = transaction_repo
//                 .get_multi_txs_by_portfolio_id_asset_id(pa.portfolio_id, &pa.asset_id)
//                 .await?;
//             let transactions: Vec<Transaction> = tx_rows
//                 .into_iter()
//                 .map(|tx| Transaction::from_row(tx))
//                 .collect();
//             pa.transactions = transactions;
//         }
//         let mut portfolio = Portfolio::from_row(portfolio_row);
//         portfolio.assets = all_portfolio_assets;
//         Ok(Some(portfolio))
//     }
// }
