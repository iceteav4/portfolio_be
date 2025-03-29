use crate::models::domain::auth::Claims;
use crate::models::dto::{api_response::ApiResponse, user::UserMeResponse};
use crate::state::AppState;
use crate::{db::repositories::user::UserRepository, models::dto::user::UserResponse};
use axum::Extension;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use time::format_description::well_known::Rfc3339;

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
    let user_repo = UserRepository::new(Arc::new(state.pool.clone()));
    let user = user_repo.get_by_id(claims.user_id).await;
    match user {
        Ok(Some(user)) => {
            return ApiResponse::success(UserMeResponse {
                id: user.id,
                status: user.status,
                email: user.email,
                phone_number: user.phone_number,
                name: user.name,
                created_at: user.created_at.format(&Rfc3339).unwrap(),
            });
        }
        Ok(None) => return ApiResponse::error(StatusCode::NOT_FOUND, "User not found"),
        Err(e) => return ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
