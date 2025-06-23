use axum::Extension;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::info;

use crate::models::domain::auth::Claims;
use crate::models::dto::{api_response::ApiResponse, user::UserMeResponse};
use crate::state::AppState;
use crate::{db::repositories::user::UserRepo, models::dto::user::UserResponse};

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
    Path(id): Path<String>,
) -> ApiResponse<UserResponse> {
    info!("Get user by id {}", id);
    let user = UserRepo::new(state.pool.clone())
        .get_by_id(id.parse().unwrap())
        .await;
    match user {
        Ok(Some(user)) => ApiResponse::success(UserResponse {
            id: user.id.to_string(),
            name: user.name,
        }),
        Ok(None) => ApiResponse::error(StatusCode::NOT_FOUND, "User not found"),
        Err(e) => ApiResponse::from(e),
    }
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "User found", body = ApiResponse<UserMeResponse>),
))]
pub async fn get_user_me(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> ApiResponse<UserMeResponse> {
    info!("user_id in ctx {}", claims.user_id);
    let user_repo = UserRepo::new(state.pool.clone());
    let user = user_repo.get_by_id(claims.user_id).await;
    match user {
        Ok(Some(user)) => {
            return ApiResponse::success(UserMeResponse {
                id: user.id.to_string(),
                status: user.status.parse().unwrap(),
                email: user.email,
                phone_number: user.phone_number,
                name: user.name,
                created_at: user.created_at,
            });
        }
        Ok(None) => return ApiResponse::error(StatusCode::NOT_FOUND, "User not found"),
        Err(e) => return ApiResponse::from(e),
    }
}
