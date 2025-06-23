use axum::Json;
use axum::http::StatusCode;
use axum::{
    Extension,
    extract::{Query, State},
};

use crate::models::dto::api_response::IdResponse;
use crate::{
    db::repositories::asset::AssetRepo,
    models::{
        common::asset::AssetType,
        domain::auth::Claims,
        dto::{
            api_response::ApiResponse,
            asset::{
                AssetListResponse, AssetQueryParams, AssetResponse, CreateAssetRepo,
                CreateAssetRequest,
            },
        },
    },
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/api/assets",
    responses(
        (status = 200, description = "Success", body = ApiResponse<AssetListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_assets(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(params): Query<AssetQueryParams>,
) -> ApiResponse<AssetListResponse> {
    let asset_repo = AssetRepo::new(state.pool.clone());
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(50);
    let asset_type = match params.asset_type {
        Some(s) => Some(s.to_string()),
        None => None,
    };
    let assets = asset_repo
        .get_multi_with_paging(asset_type, page, limit)
        .await;
    if let Err(err) = assets {
        return ApiResponse::from(err);
    }
    let assets = assets.unwrap();
    return ApiResponse::success(AssetListResponse {
        items: assets
            .into_iter()
            .map(|item| AssetResponse::from_asset_row(item))
            .collect(),
    });
}

#[utoipa::path(
    post,
    path = "/api/assets",
    request_body = CreateAssetRequest,
    responses(
        (status = 200, description = "Success", body = ApiResponse<IdResponse>),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_asset(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<CreateAssetRequest>,
) -> ApiResponse<IdResponse> {
    if req.asset_type != AssetType::Crypto {
        return ApiResponse::error(StatusCode::BAD_REQUEST, "Invalid asset type".to_string());
    }
    let cg_res = state
        .clients
        .coingecko
        .get_coin_data(&req.external_id)
        .await;
    if let Err(e) = cg_res {
        return ApiResponse::from(e);
    }
    let coin_data = cg_res.unwrap();
    let asset_repo = AssetRepo::new(state.pool.clone());
    let result = asset_repo
        .create_one(CreateAssetRepo::from_coin_data(coin_data))
        .await;
    if let Err(e) = result {
        return ApiResponse::from(e);
    }
    let asset_id = result.unwrap();
    return ApiResponse::success(IdResponse { id: asset_id });
}
