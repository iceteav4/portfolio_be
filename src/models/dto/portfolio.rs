use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::entities::portfolio::Portfolio;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePortfolioRequest {
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BriefPortfolioResponse {
    pub id: String,
    pub name: String,
}
impl BriefPortfolioResponse {
    pub fn from_entity(entity: Portfolio) -> Self {
        Self {
            id: entity.id.to_string(),
            name: entity.name,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BriefPortfolioListResponse {
    pub items: Vec<BriefPortfolioResponse>,
}
