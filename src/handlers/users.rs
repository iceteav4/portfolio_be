use crate::models::api_response::ApiResponse;
use crate::state::AppState;
use crate::{db::repositories::user::UserRepository, models::user::UserResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResponse<UserResponse> {
    let user = UserRepository::new(Arc::new(state.pg_pool.clone()))
        .get_by_id(id)
        .await;
    match user {
        Ok(Some(user)) => ApiResponse::success(UserResponse {
            id: user.id,
            name: user.name,
        }),
        Ok(None) => {
            let mut response =
                ApiResponse::error(StatusCode::NOT_FOUND, "User not found".to_string());
            response.status_code = StatusCode::NOT_FOUND.as_u16();
            response
        }
        Err(e) => {
            let mut response = ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get user: {}", e),
            );
            response.status_code = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
            response
        }
    }
}
