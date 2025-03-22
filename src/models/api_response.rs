use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub message: Option<String>,
    pub status_code: u16,
    pub unix_time: u64,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            message: None,
            status_code: 200,
            unix_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            data: Some(data),
        }
    }

    pub fn error(status_code: StatusCode, message: impl Into<String>) -> ApiResponse<T> {
        Self {
            message: Some(message.into()),
            status_code: status_code.as_u16(),
            unix_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status = if self.status_code >= 500 {
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        } else {
            StatusCode::OK
        };
        (status, Json(self)).into_response()
    }
}
