use super::{crypto_asset::CryptoAsset, stock_asset::StockAsset};
use crate::models::entities::transaction::{Transaction, TxType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum PortfolioAsset {
    Crypto(CryptoAsset),
    Stock(StockAsset),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetPosition {
    pub asset: PortfolioAsset,
    pub transactions: Vec<Transaction>,
}

impl AssetPosition {
    pub fn total_buy_value(&self) -> Decimal {
        self.transactions
            .iter()
            .filter(|t| t.tx_type == TxType::Buy)
            .map(|t| t.price * t.quantity)
            .sum()
    }

    pub fn total_sell_value(&self) -> Decimal {
        self.transactions
            .iter()
            .filter(|t| t.tx_type == TxType::Sell)
            .map(|t| t.price * t.quantity)
            .sum()
    }

    pub fn average_buy_price(&self) -> Decimal {
        let buys: Vec<_> = self
            .transactions
            .iter()
            .filter(|t| t.tx_type == TxType::Buy)
            .collect();

        if buys.is_empty() {
            return Decimal::new(0, 0);
        }

        let total_quantity: Decimal = buys.iter().map(|t| t.quantity).sum();
        let total_value: Decimal = buys.iter().map(|t| t.price * t.quantity).sum();

        total_value / total_quantity
    }
}
