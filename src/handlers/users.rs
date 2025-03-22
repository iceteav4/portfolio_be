use crate::models::api_response::ApiResponse;
use crate::state::AppState;
use crate::{db::repositories::user::UserRepository, models::user::UserResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User found", body = ApiResponse<UserResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResponse<UserResponse> {
    let user = UserRepository::new(Arc::new(state.pool.clone()))
        .get_by_id(id)
        .await;
    match user {
        Ok(Some(user)) => ApiResponse::success(UserResponse {
            id: user.id,
            name: user.name,
        }),
        Ok(None) => ApiResponse::error(StatusCode::NOT_FOUND, "User not found"),
        Err(e) => ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
