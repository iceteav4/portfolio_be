use axum::{Json, extract::State};
use serde_json::json;
use time::OffsetDateTime;

use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> Json<serde_json::Value> {
    // Access database pool from a global state or singleton
    // This assumes you have a way to access the database connection without state parameter
    let db_pool = state.pg_pool.clone();

    let db_status = sqlx::query("SELECT 1").fetch_one(&db_pool).await.is_ok();

    Json(json!({
        "status": "ok",
        "db_status": db_status,
        "timestamp": OffsetDateTime::now_utc().unix_timestamp(),
    }))
}
