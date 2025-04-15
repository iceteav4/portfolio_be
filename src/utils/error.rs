use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::Error as SerdeError;
use sqlx::Error as SqlxError;
use strum::Display;
use tracing::error;

use crate::models::dto::api_response::ApiResponse;

#[derive(Debug, Display)]
pub enum AppError {
    SqlError(SqlxError),
    SerdeError(SerdeError),
    CoinGeckoError(String),
    Unauthorized(String),
    InternalServerError,
}

impl AppError {
    pub fn get_status_code_and_error_msg(self) -> (StatusCode, String) {
        match self {
            AppError::SqlError(err) => {
                error!("SQL error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            AppError::SerdeError(err) => {
                error!("Serde error: {}", err);
                (StatusCode::BAD_REQUEST, "Invalid data".to_string())
            }
            AppError::CoinGeckoError(msg) => {
                error!("CoinGecko error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = self.get_status_code_and_error_msg();
        ApiResponse::<()>::error(status, error_message).into_response()
    }
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::SqlError(err)
    }
}

impl From<SerdeError> for AppError {
    fn from(value: SerdeError) -> Self {
        AppError::SerdeError(value)
    }
}
