use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::EnumString;
use strum_macros::{AsRefStr, Display};

#[derive(Debug, Serialize, Deserialize, Type, EnumString, AsRefStr, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Currency {
    Usd,
    Vnd,
}
