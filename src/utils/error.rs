use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed")]
    AuthError,

    #[error("Resource not found")]
    NotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal server error")]
    InternalServerError,

    // #[error("Transaction error: {0}")]
    // TransactionError(#[from] crate::utils::coingecko_exporter::TransactionError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Unauthorized")]
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AuthError => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::DatabaseError(e) => {
                // Log the detailed error internally
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred".to_string(),
                )
            }
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            // AppError::TransactionError(e) => {
            //     tracing::error!("Transaction error: {:?}", e);
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         "Failed to process transaction data".to_string(),
            //     )
            // }
            AppError::IoError(e) => {
                tracing::error!("IO error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "File operation failed".to_string(),
                )
            }
            AppError::SerializationError(e) => {
                tracing::error!("Serialization error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to process data".to_string(),
                )
            }
            AppError::ConfigError(msg) => {
                tracing::error!("Configuration error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Application configuration error".to_string(),
                )
            }
            AppError::JwtError(e) => {
                tracing::error!("JWT error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to process JWT".to_string(),
                )
            }
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
        };

        let body = json!({
            "error": error_message,
        });

        (status, axum::Json(body)).into_response()
    }
}
