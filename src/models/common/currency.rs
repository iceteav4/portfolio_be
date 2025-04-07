use serde::{Deserialize, Serialize};
use sqlx::Type;
#[derive(Debug, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum Currency {
    Usd,
    Vnd,
}

impl Currency {
    pub fn as_str(&self) -> &str {
        match self {
            Currency::Usd => "usd",
            Currency::Vnd => "vnd",
        }
    }

    pub fn from_str(s: &str) -> Option<Currency> {
        match s {
            "usd" => Some(Currency::Usd),
            "vnd" => Some(Currency::Vnd),
            _ => None,
        }
    }
}
