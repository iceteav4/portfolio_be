use axum::Json;
use axum::http::StatusCode;
use axum::{
    Extension,
    extract::{Query, State},
};

use crate::biz::asset::generate_asset_id;
use crate::models::dto::api_response::IdResponse;
use crate::to_api_res;
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
    params(AssetQueryParams),
    responses(
        (status = 200, description = "Success", body = ApiResponse<AssetListResponse>)
    )
)]
pub async fn get_all_assets(
    State(state): State<AppState>,
    Extension(_claims): Extension<Claims>,
    Query(params): Query<AssetQueryParams>,
) -> ApiResponse<AssetListResponse> {
    let asset_repo = AssetRepo::new(state.pool.clone());
    let asset_type = params.asset_type.map(|t| t.to_string());
    let assets = to_api_res!(
        asset_repo
            .get_multi_with_paging(asset_type, params.page, params.limit)
            .await
    );
    return ApiResponse::success(AssetListResponse {
        items: assets
            .into_iter()
            .map(|item| AssetResponse::from_db_row(item))
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
    let coin_data = to_api_res!(
        state
            .clients
            .coingecko
            .get_coin_data(&req.external_id)
            .await
    );
    let asset_repo = AssetRepo::new(state.pool.clone());
    let asset_id = generate_asset_id(&AssetType::Crypto, &coin_data.id);
    let existed_asset = asset_repo.get_one_by_id(&asset_id).await;
    match existed_asset {
        Err(e) => return ApiResponse::from(e),
        Ok(Some(_)) => {
            return ApiResponse::<IdResponse>::error(
                StatusCode::BAD_REQUEST,
                "Asset already exists",
            );
        }
        Ok(None) => (),
    }
    let asset_id = to_api_res!(
        asset_repo
            .create_one(CreateAssetRepo::from_coin_data(coin_data))
            .await
    );
    return ApiResponse::success(IdResponse { id: asset_id });
}
