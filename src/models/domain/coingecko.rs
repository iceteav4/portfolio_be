use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawTransaction {
    id: u32,
    transaction_type: String,
    currency: String,
    quantity: String,
    price: String,
    transaction_timestamp: String,
    fees: String,
    notes: String,
}
