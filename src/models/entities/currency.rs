use serde::{Deserialize, Serialize};
use sqlx::Type;
#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "currency", rename_all = "lowercase")]
pub enum Currency {
    Usd,
    Vnd,
}
