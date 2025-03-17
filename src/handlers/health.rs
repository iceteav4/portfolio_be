use axum::{Json, extract::State};
use serde_json::json;
use std::sync::Arc;
use time::OffsetDateTime;

use crate::state::AppState;

pub async fn health_check(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let db_status = sqlx::query("SELECT 1")
        .fetch_one(&state.pg_pool)
        .await
        .is_ok();
    Json(json!({
        "status": "ok",
        "db_status": db_status,
        "timestamp": OffsetDateTime::now_utc().unix_timestamp(),
    }))
}
