use serde::{Deserialize, Serialize};
use strum::EnumString;
use strum_macros::Display;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, EnumString, Display, ToSchema)]
#[strum(serialize_all = "lowercase")]
pub enum Currency {
    Usd,
    Vnd,
}
