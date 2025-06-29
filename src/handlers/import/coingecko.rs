use std::collections::HashMap;

use axum::{
    Extension,
    extract::{Multipart, Path, State},
    http::StatusCode,
};
use tracing::info;

use crate::{
    biz::asset::generate_asset_id,
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
            asset::CreateAssetRepo,
            coingecko::CoinDataResponse,
            transaction::{CreateMultiTransaction, UpdateTransaction},
        },
    },
    state::AppState,
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
        Ok(portfolio) => match portfolio {
            Some(p) => {
                if p.owner_id != claims.user_id {
                    info!("Portfolio owner id does not match");
                    return ApiResponse::error(
                        StatusCode::BAD_REQUEST,
                        "Portfolio owner id does not match".to_string(),
                    );
                }
            }
            None => {
                info!("Portfolio not found");
                return ApiResponse::error(
                    StatusCode::NOT_FOUND,
                    "Portfolio not found".to_string(),
                );
            }
        },
        Err(e) => {
            info!("Failed to get portfolio id {}", portfolio_id);
            return ApiResponse::from(e);
        }
    }
    let coin_data = state.clients.coingecko.get_coin_data(&coin_id).await;
    if let Err(e) = coin_data {
        info!("Failed to get coin data");
        return ApiResponse::from(e);
    }
    let coin_data = coin_data.unwrap();
    // create new asset if needed
    let asset_id = generate_asset_id(&AssetType::Crypto, &coin_data.id);
    let existed_asset = asset_repo.get_one_by_id(&asset_id).await;
    if let Err(e) = existed_asset {
        info!("Failed to get asset");
        return ApiResponse::from(e);
    }
    let existed_asset = existed_asset.unwrap();
    if existed_asset.is_none() {
        info!("Asset does not exist, create new crypto asset");
        let new_asset = asset_repo
            .create_one(CreateAssetRepo::from_coin_data(coin_data))
            .await;
        if let Err(e) = new_asset {
            info!("Failed to create asset");
            return ApiResponse::from(e);
        }
        info!("New crypto asset was created with id: {}", asset_id);
    }
    // create portfolio asset
    let pa_repo = PortfolioAssetRepo::new(state.pool.clone());
    let exist_pa = pa_repo
        .get_one_by_portfolio_id_and_asset_id(portfolio_id, &asset_id)
        .await;
    match exist_pa {
        Err(e) => {
            info!(
                "Failed to checking exist portfolio_id {} and asset_id {}",
                portfolio_id, &asset_id
            );
            return ApiResponse::from(e);
        }
        Ok(None) => {
            let portfolio_asset = pa_repo.create(portfolio_id, &asset_id).await;
            if let Err(e) = portfolio_asset {
                info!("Failed to create portfolio asset");
                return ApiResponse::from(e);
            }
            let portfolio_asset = portfolio_asset.unwrap();
            info!(
                "Created portfolio asset with portfolio id: {}, asset id: {}",
                portfolio_asset.portfolio_id, portfolio_asset.asset_id
            );
        }
        _ => {}
    }
    // save txs when have new tx
    let all_pa_txs = tx_repo
        .get_multi_txs_by_portfolio_id_asset_id(portfolio_id, &asset_id)
        .await;
    if let Err(e) = all_pa_txs {
        info!("Failed to get transactions");
        return ApiResponse::from(e);
    }
    let all_pa_txs = all_pa_txs.unwrap();
    let external_id_to_tx: HashMap<String, TransactionRow> = all_pa_txs
        .into_iter()
        .filter_map(|tx| tx.external_id.clone().map(|external_id| (external_id, tx)))
        .collect();
    let mut new_txs = Vec::new();
    for raw_tx in new_raw_txs.into_iter() {
        let base_tx_info = BaseTransactionInfo::from_raw_tx(raw_tx);
        if let Err(e) = base_tx_info {
            info!("Failed to convert raw transaction to base tx info: {}", e);
            return ApiResponse::from(e);
        }
        let base_tx_info = base_tx_info.unwrap();
        if base_tx_info.external_id.is_none() {
            return ApiResponse::error(
                StatusCode::BAD_REQUEST,
                "Transaction from Coingecko does not have id",
            );
        }
        let external_id = base_tx_info.external_id.clone().unwrap();
        if let Some(tx) = external_id_to_tx.get(&external_id) {
            // external transaction existed, update current transaction
            let update_result = tx_repo
                .update_tx_by_id(
                    tx.id,
                    UpdateTransaction {
                        tx_type: Some(base_tx_info.tx_type),
                        notes: base_tx_info.notes,
                        quantity: Some(base_tx_info.quantity),
                        price: Some(base_tx_info.price),
                        fees: Some(base_tx_info.fees),
                        executed_at: Some(base_tx_info.executed_at),
                    },
                )
                .await;
            if let Err(e) = update_result {
                info!("Failed to update transaction: {}", e);
                return ApiResponse::from(e);
            }
        } else {
            new_txs.push(base_tx_info);
        }
    }
    let create_multi_txs = CreateMultiTransaction {
        portfolio_id,
        asset_id,
        transactions: new_txs,
    };

    let new_txs = tx_repo.create_multi_txs(create_multi_txs).await;
    if let Err(e) = new_txs {
        info!("Failed to create multi transactions");
        return ApiResponse::from(e);
    }
    let new_txs = new_txs.unwrap();
    info!("Total transactions created: {}", new_txs);

    return ApiResponse::<GeneralResponse>::success_general_response();
}
