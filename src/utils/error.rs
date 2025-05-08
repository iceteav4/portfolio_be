use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rust_decimal::Error as DecimalError;
use serde_json::Error as SerdeError;
use sqlx::Error as SqlxError;
use strum::Display;
use strum::ParseError as StrumParseError;
use time::error::{Format as TimeFormatError, Parse as TimeParseError};
use tracing::error;

use crate::models::dto::api_response::ApiResponse;

#[derive(Debug, Display)]
pub enum AppError {
    SqlError(SqlxError),
    SerdeError(SerdeError),
    CoinGeckoError(String),
    HttpError(String),
    Unauthorized(String),
    InternalServerError,
    TimeParseError(TimeParseError),
    TimeFormatError(TimeFormatError),
    StrumParseError(StrumParseError),
    DecimalError(DecimalError),
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
            AppError::HttpError(msg) => {
                error!("HTTP error: {}", msg);
                (
                    StatusCode::BAD_GATEWAY,
                    "Error communicating with external service".to_string(),
                )
            }
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ),
            AppError::TimeParseError(err) => {
                error!("Time parse error: {}", err);
                (StatusCode::BAD_REQUEST, "Invalid datetime format".into())
            }
            AppError::TimeFormatError(err) => {
                error!("Time format error: {}", err);
                (StatusCode::BAD_REQUEST, "Error formatting datetime".into())
            }
            AppError::StrumParseError(err) => {
                error!("Strum parse error: {}", err);
                (StatusCode::BAD_REQUEST, "Invalid enum value".into())
            }
            AppError::DecimalError(err) => {
                error!("Decimal error: {}", err);
                (StatusCode::BAD_REQUEST, "Invalid decimal value".into())
            }
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

impl From<TimeParseError> for AppError {
    fn from(err: TimeParseError) -> Self {
        AppError::TimeParseError(err)
    }
}

impl From<TimeFormatError> for AppError {
    fn from(err: TimeFormatError) -> Self {
        AppError::TimeFormatError(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpError(err.to_string())
    }
}

impl From<StrumParseError> for AppError {
    fn from(err: StrumParseError) -> Self {
        AppError::StrumParseError(err)
    }
}

impl From<DecimalError> for AppError {
    fn from(err: DecimalError) -> Self {
        AppError::DecimalError(err)
    }
}
