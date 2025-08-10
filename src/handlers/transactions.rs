use axum::extract::{Query, State};

use crate::{
    models::dto::{
        api_response::ApiResponse,
        pagination::CursorPaginationResponse,
        transaction::{TransactionListResponse, TransactionQueryParams},
    },
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/api/transactions",
    responses(
        (status = 200, description = "Success", body = ApiResponse<TransactionListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transactions(
    State(state): State<AppState>,
    Query(params): Query<TransactionQueryParams>,
) -> ApiResponse<TransactionListResponse> {
    return ApiResponse::success(TransactionListResponse {
        cursor_pagination: CursorPaginationResponse {
            end_cursor: None,
            has_next: false,
            total_items: None,
        },
        items: Vec::new(),
    });
}
