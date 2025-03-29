use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
    pub status_code: u16,
}
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub unix_time: u64,
    pub errors: Vec<ErrorResponse>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            unix_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            errors: vec![],
            data: Some(data),
        }
    }

    pub fn error(status_code: StatusCode, message: impl Into<String>) -> ApiResponse<T> {
        Self {
            unix_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            errors: vec![ErrorResponse {
                message: message.into(),
                status_code: status_code.as_u16(),
            }],
            data: None,
        }
    }
    #[allow(dead_code)]
    pub fn errors(errors: Vec<ErrorResponse>) -> ApiResponse<T> {
        Self {
            unix_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            errors,
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
