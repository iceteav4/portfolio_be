use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::{AsRefStr, EnumString};
#[derive(Debug, Serialize, Deserialize, Type, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Currency {
    Usd,
    Vnd,
}
