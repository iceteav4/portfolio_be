use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
use strum_macros::Display;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, EnumString, EnumIter, Display, ToSchema)]
#[strum(serialize_all = "lowercase")]
pub enum Currency {
    USD,
    VND,
}
