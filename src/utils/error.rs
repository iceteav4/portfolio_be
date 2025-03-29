use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::models::dto::api_response::ApiResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("CoinGeckoError")]
    CoinGeckoError(String),

    // #[error("Transaction error: {0}")]
    // TransactionError(#[from] crate::utils::coingecko_exporter::TransactionError),
    #[error("Unauthorized")]
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(e) => {
                // Log the detailed error internally
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred".to_string(),
                )
            }
            // AppError::TransactionError(e) => {
            //     tracing::error!("Transaction error: {:?}", e);
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         "Failed to process transaction data".to_string(),
            //     )
            // }
            AppError::CoinGeckoError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
        };
        ApiResponse::<()>::error(status, error_message).into_response()
    }
}
