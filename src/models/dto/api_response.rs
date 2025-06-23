use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use time::OffsetDateTime;
use utoipa::ToSchema;

use crate::utils::datetime::serialize_datetime;
use crate::utils::error::AppError;

#[derive(Serialize, ToSchema)]
pub struct GeneralResponse {
    pub success: bool,
}

#[derive(Serialize, ToSchema)]
pub struct IdResponse {
    pub id: String,
}

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
    #[serde(serialize_with = "serialize_datetime")]
    pub server_time: OffsetDateTime,
    pub errors: Option<Vec<ErrorResponse>>,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            server_time: OffsetDateTime::now_utc(),
            errors: None,
            data: Some(data),
        }
    }

    pub fn success_general_response() -> ApiResponse<GeneralResponse> {
        ApiResponse {
            server_time: OffsetDateTime::now_utc(),
            errors: None,
            data: Some(GeneralResponse { success: true }),
        }
    }

    pub fn error(status_code: StatusCode, message: impl Into<String>) -> ApiResponse<T> {
        Self {
            server_time: OffsetDateTime::now_utc(),
            errors: Some(vec![ErrorResponse {
                message: message.into(),
                status_code: status_code.as_u16(),
            }]),
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn errors(errors: Vec<ErrorResponse>) -> ApiResponse<T> {
        Self {
            server_time: OffsetDateTime::now_utc(),
            errors: Some(errors),
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

impl<T> From<AppError> for ApiResponse<T>
where
    T: Serialize,
{
    fn from(err: AppError) -> Self {
        let (status_code, error_msg) = err.get_status_code_and_error_msg();
        Self {
            server_time: OffsetDateTime::now_utc(),
            errors: Some(vec![ErrorResponse {
                message: error_msg,
                status_code: status_code.as_u16(),
            }]),
            data: None,
        }
    }
}
