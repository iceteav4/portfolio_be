use crate::models::{api_response::ApiResponse, user::CreateUserRequest};
use crate::state::AppState;
use crate::{
    db::repositories::user::{CreateUser, UserRepository},
    models::user::UserResponse,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;

pub async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<CreateUserRequest>,
) -> ApiResponse<UserResponse> {
    let user = UserRepository::new(Arc::new(app_state.pg_pool.clone()))
        .create_user(CreateUser { name: req.name })
        .await;

    match user {
        Ok(user) => ApiResponse::success(UserResponse {
            id: user.id,
            name: user.name,
        }),
        Err(e) => {
            let mut response = ApiResponse::error(format!("Failed to create user: {}", e));
            response.status_code = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
            response
        }
    }
}

pub async fn get_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> ApiResponse<UserResponse> {
    let user = UserRepository::new(Arc::new(app_state.pg_pool.clone()))
        .find_by_id(id)
        .await;
    match user {
        Ok(Some(user)) => ApiResponse::success(UserResponse {
            id: user.id,
            name: user.name,
        }),
        Ok(None) => {
            let mut response = ApiResponse::error("User not found");
            response.status_code = StatusCode::NOT_FOUND.as_u16();
            response
        }
        Err(e) => {
            let mut response = ApiResponse::error(format!("Failed to get user: {}", e));
            response.status_code = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
            response
        }
    }
}
