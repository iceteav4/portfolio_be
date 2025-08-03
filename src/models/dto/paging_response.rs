use serde::Deserialize;
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
