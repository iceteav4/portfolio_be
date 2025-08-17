use std::collections::HashMap;

use axum::{
    Extension,
    extract::{Multipart, Path, State},
    http::StatusCode,
};
use tracing::info;

use crate::{
    biz::{asset::generate_asset_id, portfolio_asset::update_portfolio_asset_stat},
    db::repositories::{
        asset::AssetRepo, portfolio::PortfolioRepo, portfolio_asset::PortfolioAssetRepo,
        transaction::TransactionRepo,
    },
    models::{
        common::asset::AssetType,
        database::transaction::TransactionRow,
        domain::{auth::Claims, coingecko::RawTransaction, transaction::BaseTransactionInfo},
        dto::{
            api_response::{ApiResponse, GeneralResponse},
            coingecko::CoinDataResponse,
            transaction::{CreateMultiTransaction, UpdateTransaction},
        },
    },
    state::AppState,
    to_api_res,
};

#[utoipa::path(
    get,
    path = "/api/imports/coingecko/coin_data",
    responses(
        (status = 200, description = "User found", body = ApiResponse<GeneralResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_coin_data_by_id(
    State(state): State<AppState>,
    Path(coin_id): Path<String>,
) -> ApiResponse<CoinDataResponse> {
    info!("get coin data");
    let response = state.clients.coingecko.get_coin_data(&coin_id).await;
    match response {
        Ok(response) => ApiResponse::success(CoinDataResponse {
            market_data: response.market_data.limit_as_currency(),
            ..response
        }),
        Err(_) => {
            info!("Can not get coin data from CoinGecko");
            ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can not get coin data".to_string(),
            )
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/imports/coingecko/upload_portfolio_file",
    responses(
        (status = 200, description = "User found", body = ApiResponse<GeneralResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn import_portfolio_file(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> ApiResponse<GeneralResponse> {
    let asset_repo = AssetRepo::new(state.pool.clone());
    let portfolio_repo = PortfolioRepo::new(state.pool.clone());
    let tx_repo = TransactionRepo::new(state.pool.clone());
    let mut portfolio_id: Option<i64> = None;
    let mut target_coin_id: Option<String> = None;
    let mut new_raw_txs: Vec<RawTransaction> = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        // Extract field_name and file_name before moving field
        let field_name = field.name();
        let file_name = field.file_name();
        let content_type = field.content_type();
        info!(
            "Field name: {:?}, file name: {:?}, content type: {:?}",
            field_name, file_name, content_type
        );

        if let Some(ref name) = field_name {
            if name == &"file" {
                // Check if the file is an HTML file
                if let Some(ref ct) = content_type {
                    if ct != &"text/html" {
                        return ApiResponse::error(
                            StatusCode::BAD_REQUEST,
                            "Only HTML files are allowed".to_string(),
                        );
                    }
                }

                if let Some(ref fname) = file_name {
                    info!("File name: {}", fname);
                }

                // Read file content as string
                let content = field.text().await;
                if content.is_err() {
                    info!("Failed to read HTML file");
                    return ApiResponse::error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to read HTML file".to_string(),
                    );
                }
                let content = content.unwrap();
                let parse_html_result = state.clients.coingecko.parse_html_contents(&content);
                if let Err(e) = parse_html_result {
                    info!("Failed to parse HTML file");
                    return ApiResponse::from(e);
                }
                let (coin_id, raw_txs) = parse_html_result.unwrap();
                info!("Coin ID: {}", coin_id);
                info!("Raw transactions: {}", raw_txs.len());
                target_coin_id = Some(coin_id);
                new_raw_txs = raw_txs;
            } else if name == &"portfolio_id" {
                // Read the value of the portfolio_id field
                let content = field.text().await;
                if let Ok(id) = content {
                    match id.parse::<i64>() {
                        Ok(parsed_id) => {
                            portfolio_id = Some(parsed_id);
                        }
                        Err(_) => {
                            info!("portfolio_id is not a valid i64: {}", id);
                            return ApiResponse::error(
                                StatusCode::BAD_REQUEST,
                                "portfolio_id must be a valid integer".to_string(),
                            );
                        }
                    }
                } else {
                    info!("Failed to read portfolio_id field");
                    return ApiResponse::error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to read portfolio_id field".to_string(),
                    );
                }
            }
        }
    }
    if portfolio_id.is_none() || target_coin_id.is_none() {
        info!("No portfolio id or target coin id");
        return ApiResponse::error(
            StatusCode::BAD_REQUEST,
            "Portfolio ID or file upload is missing".to_string(),
        );
    }
    let portfolio_id = portfolio_id.unwrap();
    let coin_id = target_coin_id.unwrap();
    info!(
        "Portfolio ID: {}, coin ID: {}, total tx: {}",
        portfolio_id,
        coin_id,
        new_raw_txs.len()
    );
    let portfolio = portfolio_repo.get_one_by_id(portfolio_id).await;
    match portfolio {
        Ok(Some(pfl)) => {
            if pfl.owner_id != claims.user_id {
                info!("Portfolio owner id does not match");
                return ApiResponse::error(
                    StatusCode::BAD_REQUEST,
                    "Portfolio owner id does not match".to_string(),
                );
            }
        }
        Ok(None) => {
            return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio not found".to_string());
        }
        Err(e) => {
            return ApiResponse::from(e);
        }
    }
    let coin_data = to_api_res!(state.clients.coingecko.get_coin_data(&coin_id).await);
    // create new asset if needed
    let asset_id = generate_asset_id(&AssetType::Crypto, &coin_data.id);
    let existed_asset = asset_repo.get_one_by_id(&asset_id).await;
    match existed_asset {
        Err(e) => return ApiResponse::from(e),
        Ok(None) => {
            return ApiResponse::error(StatusCode::BAD_REQUEST, "Asset does not exist");
        }
        Ok(Some(_)) => (),
    }
    // create portfolio asset
    let pa_repo = PortfolioAssetRepo::new(state.pool.clone());
    let exist_pa = pa_repo
        .get_one_by_portfolio_id_and_asset_id(portfolio_id, &asset_id)
        .await;
    match exist_pa {
        Err(e) => {
            return ApiResponse::from(e);
        }
        Ok(None) => {
            return ApiResponse::error(StatusCode::BAD_REQUEST, "Portfolio asset does not exist");
        }
        _ => {}
    }
    // save txs when have new tx
    let all_pa_txs = to_api_res!(
        tx_repo
            .get_multi_txs_by_portfolio_and_asset_with_paging(portfolio_id, &asset_id, 1, 10_000)
            .await
    );
    let external_id_to_tx: HashMap<String, TransactionRow> = all_pa_txs
        .into_iter()
        .filter_map(|tx| tx.external_id.clone().map(|external_id| (external_id, tx)))
        .collect();
    let mut new_txs = Vec::new();
    for raw_tx in new_raw_txs.into_iter() {
        let base_tx_info = to_api_res!(BaseTransactionInfo::from_raw_tx(raw_tx));
        if base_tx_info.external_id.is_none() {
            return ApiResponse::error(
                StatusCode::BAD_REQUEST,
                "Transaction from Coingecko does not have id",
            );
        }
        let external_id = base_tx_info.external_id.clone().unwrap();
        if let Some(tx) = external_id_to_tx.get(&external_id) {
            // external transaction existed, update current transaction
            let update_tx = UpdateTransaction {
                tx_type: Some(base_tx_info.tx_type),
                price: Some(base_tx_info.price),
                quantity: Some(base_tx_info.quantity),
                fees: Some(base_tx_info.fees),
                currency: Some(base_tx_info.currency),
                notes: base_tx_info.notes,
                executed_at: Some(base_tx_info.executed_at),
            };
            to_api_res!(tx_repo.update_tx_by_id(tx.id, update_tx).await);
        } else {
            new_txs.push(base_tx_info);
        }
    }
    let create_multi_txs = CreateMultiTransaction {
        portfolio_id,
        asset_id: asset_id.clone(),
        transactions: new_txs,
    };
    let new_txs = to_api_res!(tx_repo.create_multi_txs(create_multi_txs).await);
    info!("Total transactions created: {}", new_txs);

    to_api_res!(update_portfolio_asset_stat(state.pool.clone(), portfolio_id, &asset_id).await);

    return ApiResponse::<GeneralResponse>::success_general_response();
}
