use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::info;

use crate::{
    models::dto::{api_response::ApiResponse, coingecko::CoinDataResponse},
    state::AppState,
};

pub async fn get_coin_data_by_id(
    State(state): State<AppState>,
    Path(coin_id): Path<String>,
) -> ApiResponse<CoinDataResponse> {
    info!("get coin data");
    let response = state.coingecko_client.get_coin_data(&coin_id).await;
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
