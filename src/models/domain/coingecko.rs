use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawTransaction {
    pub id: u32,
    pub transaction_type: String,
    pub currency: String,
    pub quantity: String,
    pub price: String,
    pub transaction_timestamp: String,
    pub fees: String,
    pub notes: Option<String>,
}
