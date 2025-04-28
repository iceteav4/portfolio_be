use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
};
use tracing::info;

use crate::{
    models::dto::{
        api_response::{ApiResponse, GeneralResponse},
        coingecko::CoinDataResponse,
    },
    state::AppState,
};

pub async fn get_coin_data_by_id(
    State(state): State<AppState>,
    Path(coin_id): Path<String>,
) -> ApiResponse<CoinDataResponse> {
    info!("get coin data");
    let response = state.clients.coingecko.get_coin_data(&coin_id).await;
    match response {
        Ok(response) => ApiResponse::success(response),
        Err(_) => {
            info!("Can not get coin data from CoinGecko");
            ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can not get coin data".to_string(),
            )
        }
    }
}

pub async fn import_portfolio_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> ApiResponse<GeneralResponse> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        // Check if the file is an HTML file
        if let Some(content_type) = field.content_type() {
            if content_type != "text/html" {
                return ApiResponse::error(
                    StatusCode::BAD_REQUEST,
                    "Only HTML files are allowed".to_string(),
                );
            }
        }

        info!("File name: {}", field.file_name().unwrap());

        // Read file content as string
        match field.text().await {
            Ok(content) => {
                info!("Successfully read HTML file content");
                match state.clients.coingecko.parse_html_contents(&content) {
                    Ok(_) => return ApiResponse::success(GeneralResponse { success: true }),
                    Err(e) => return ApiResponse::from(e),
                }
            }
            Err(e) => {
                info!("Failed to read HTML file: {}", e);
                return ApiResponse::error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to read HTML file".to_string(),
                );
            }
        }
    }

    ApiResponse::error(StatusCode::BAD_REQUEST, "No file was uploaded".to_string())
}
