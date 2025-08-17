use serde::Serialize;
use utoipa::ToSchema;

pub fn default_page() -> u32 {
    1
}

pub fn default_limit() -> u32 {
    10
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CursorPaginationResponse {
    pub end_cursor: Option<String>,
    pub has_next: bool,
    pub total_items: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NumberPaginationResponse {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u32,
    pub page_size: u32,
}
impl NumberPaginationResponse {
    pub fn new(current_page: u32, page_size: u32, total_items: u32) -> Self {
        let total_pages = (total_items + page_size - 1) / page_size;
        Self {
            current_page,
            total_pages,
            total_items,
            page_size,
        }
    }
}
