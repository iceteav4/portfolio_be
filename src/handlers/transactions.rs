use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::models::dto::api_response::IdResponse;
use crate::models::dto::pagination::NumberPaginationResponse;
use crate::models::dto::transaction::{
    CreateMultiTransaction, CreateTransactionRequest, TransactionResponse,
};
use crate::models::{domain::auth::Claims, dto::transaction::UpdateTransaction};
use crate::models::{domain::transaction::BaseTransactionInfo, dto::api_response::GeneralResponse};
use crate::{
    biz::portfolio_asset::update_portfolio_asset_stat,
    db::repositories::transaction::TransactionRepo,
};
use crate::{
    db::repositories::portfolio::PortfolioRepo, models::dto::transaction::UpdateTransactionRequest,
};
use crate::{
    db::repositories::portfolio_asset::PortfolioAssetRepo,
    models::dto::{
        api_response::ApiResponse,
        transaction::{TransactionListResponse, TransactionQueryParams},
    },
    state::AppState,
    to_api_res,
};
use tracing::info;

#[utoipa::path(
    post,
    path = "/api/transactions",
    responses(
        (status = 200, description = "Success", body = ApiResponse<IdResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_transaction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateTransactionRequest>,
) -> ApiResponse<IdResponse> {
    info!("Create transaction with body request {:?}", req);
    let pa_repo = PortfolioAssetRepo::new(state.pool.clone());
    let pfl_id: i64 = match req.portfolio_id.parse() {
        Ok(v) => v,
        Err(_) => {
            return ApiResponse::error(
                StatusCode::BAD_REQUEST,
                format!("Invalid portfolio ID: {}", req.portfolio_id),
            );
        }
    };
    let asset_id = req.asset_id.clone();
    let existing_pa = to_api_res!(
        pa_repo
            .get_one_by_portfolio_id_and_asset_id(pfl_id, &asset_id)
            .await
    );
    if existing_pa.is_none() {
        return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio asset not found");
    }

    let pfl_repo = PortfolioRepo::new(state.pool.clone());
    let pfl_row = to_api_res!(pfl_repo.get_one_by_id(pfl_id).await);
    match pfl_row {
        None => return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio not found"),
        Some(row) => {
            if row.owner_id != claims.user_id {
                return ApiResponse::error(StatusCode::UNAUTHORIZED, "Unauthorized");
            }
        }
    }

    let base_tx = to_api_res!(BaseTransactionInfo::from_create_tx_req(req));
    let tx_id = base_tx.id.unwrap();
    let tx_repo = TransactionRepo::new(state.pool.clone());
    to_api_res!(
        tx_repo
            .create_multi_txs(CreateMultiTransaction {
                portfolio_id: pfl_id,
                asset_id: asset_id.clone(),
                transactions: vec![base_tx]
            })
            .await
    );

    to_api_res!(update_portfolio_asset_stat(state.pool.clone(), pfl_id, &asset_id).await);

    ApiResponse::success(IdResponse {
        id: tx_id.to_string(),
    })
}

#[utoipa::path(
    get,
    path = "/api/transactions",
    params(TransactionQueryParams),
    responses(
        (status = 200, description = "Success", body = ApiResponse<TransactionListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_transactions(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(params): Query<TransactionQueryParams>,
) -> ApiResponse<TransactionListResponse> {
    let pa_repo = PortfolioAssetRepo::new(state.pool.clone());
    let pfl_id: i64 = params.portfolio_id.parse().unwrap();
    let existing_pa = to_api_res!(
        pa_repo
            .get_one_by_portfolio_id_and_asset_id(pfl_id, &params.asset_id)
            .await
    );
    if existing_pa.is_none() {
        return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio asset not found");
    }
    let tx_repo = TransactionRepo::new(state.pool.clone());
    let tx_rows = to_api_res!(
        tx_repo
            .get_multi_txs_by_portfolio_and_asset_with_paging(
                pfl_id,
                &params.asset_id,
                params.page,
                params.limit
            )
            .await
    );
    let txs_response: Vec<TransactionResponse> = tx_rows
        .into_iter()
        .map(|row| TransactionResponse::from_db_row(row))
        .collect();
    let total_items = to_api_res!(
        tx_repo
            .count_txs_by_portfolio_and_asset(pfl_id, &params.asset_id)
            .await
    );
    let total_items = total_items.unwrap_or(0);
    let response = TransactionListResponse {
        number_pagination: NumberPaginationResponse::new(
            params.page,
            params.limit,
            total_items as u32,
        ),
        items: txs_response,
    };
    return ApiResponse::success(response);
}

#[utoipa::path(
    get,
    path = "/api/transactions/{transaction_id}",
    responses(
        (status = 200, description = "Success", body = ApiResponse<TransactionResponse>),
    )
)]
pub async fn get_detail_transaction(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Path(tx_id): Path<String>,
) -> ApiResponse<TransactionResponse> {
    let tx_id: i64 = match tx_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return ApiResponse::error(
                StatusCode::BAD_REQUEST,
                format!("Invalid transaction id {}", tx_id),
            );
        }
    };
    let tx_repo = TransactionRepo::new(state.pool.clone());
    let tx_row = to_api_res!(tx_repo.get_one_by_id(tx_id).await);
    if tx_row.is_none() {
        return ApiResponse::error(StatusCode::NOT_FOUND, "Transaction not found");
    }
    let response = TransactionResponse::from_db_row(tx_row.unwrap());
    return ApiResponse::success(response);
}

#[utoipa::path(
    patch,
    path = "/api/transactions/{transaction_id}",
    responses(
        (status = 200, description = "Success", body = ApiResponse<GeneralResponse>),
    )
)]
pub async fn update_transaction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(tx_id): Path<String>,
    Json(req): Json<UpdateTransactionRequest>,
) -> ApiResponse<GeneralResponse> {
    let tx_id: i64 = match tx_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return ApiResponse::error(
                StatusCode::BAD_REQUEST,
                format!("Invalid transaction id {}", tx_id),
            );
        }
    };
    let tx_repo = TransactionRepo::new(state.pool.clone());
    let tx_row = to_api_res!(tx_repo.get_one_by_id(tx_id).await);
    let tx_row = match tx_row {
        Some(v) => v,
        None => return ApiResponse::error(StatusCode::NOT_FOUND, "Transaction not found"),
    };

    let pfl_repo = PortfolioRepo::new(state.pool.clone());
    let pfl_row = to_api_res!(pfl_repo.get_one_by_id(tx_row.portfolio_id).await);
    match pfl_row {
        Some(v) => {
            if v.owner_id != claims.user_id {
                return ApiResponse::error(StatusCode::FORBIDDEN, "Unauthorized");
            }
        }
        None => return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio not found"),
    };

    to_api_res!(
        tx_repo
            .update_tx_by_id(tx_id, UpdateTransaction::from_req(req))
            .await
    );

    to_api_res!(
        update_portfolio_asset_stat(state.pool.clone(), tx_row.portfolio_id, &tx_row.asset_id)
            .await
    );

    return ApiResponse::<GeneralResponse>::success_general_response();
}
