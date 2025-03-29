use crate::{
    models::dto::{api_response::ApiResponse, health::HealthResponse},
    state::AppState,
};
use axum::extract::State;
use time::OffsetDateTime;

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Server is healthy", body = ApiResponse<HealthResponse>),
    )
)]
pub async fn health_check(State(state): State<AppState>) -> ApiResponse<HealthResponse> {
    // Access database pool from a global state or singleton
    // This assumes you have a way to access the database connection without state parameter
    let pool = state.pool.clone();

    let db_status = sqlx::query("SELECT 1").fetch_one(&pool).await.is_ok();

    ApiResponse::success(HealthResponse {
        status: "ok".to_string(),
        db_status,
        timestamp: OffsetDateTime::now_utc().unix_timestamp() as u64,
    })
}
