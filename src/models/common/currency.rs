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
}

impl ToString for Currency {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl From<String> for Currency {
    fn from(value: String) -> Self {
        match value.as_str() {
            "usd" => Currency::Usd,
            "vnd" => Currency::Vnd,
            _ => Currency::Usd,
        }
    }
}
