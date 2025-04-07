use super::asset::crypto::CryptoAsset;
use super::asset::stock::StockAsset;
use super::transaction::Transaction;
use super::transaction::TxType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Portfolio {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub positions: HashMap<String, AssetPosition>, // key is asset.id()
}

impl Portfolio {
    pub fn total_value(&self) -> Decimal {
        self.positions
            .values()
            .map(|pos| pos.total_buy_value() - pos.total_sell_value())
            .sum()
    }
}
