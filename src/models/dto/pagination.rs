use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[allow(dead_code)]
pub struct NumberPaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct CursorPaginationQuery {
    pub after: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CursorPaginationResponse {
    pub end_cursor: Option<String>,
    pub has_next: bool,
    pub total_items: Option<u32>,
}
