use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::info;

use crate::{
    db::repositories::{
        asset::AssetRepo, portfolio::PortfolioRepo, portfolio_asset::PortfolioAssetRepo,
    },
    models::{
        domain::auth::Claims,
        dto::{
            api_response::{ApiResponse, GeneralResponse, IdResponse},
            asset::AssetResponse,
            portfolio::{
                BriefPortfolioListResponse, BriefPortfolioResponse, CreatePortfolioAssetRequest,
                CreatePortfolioRequest, PortfolioResponse,
            },
        },
    },
    state::AppState,
};

#[utoipa::path(
    post,
    path = "/api/portfolios",
    responses(
        (status = 200, description = "Success", body = ApiResponse<IdResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_portfolio(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreatePortfolioRequest>,
) -> ApiResponse<IdResponse> {
    info!("Create portfolio with body request {:?}", req);
    let portfolio_repo = PortfolioRepo::new(state.pool.clone());
    let new_portfolio = portfolio_repo.create_one(claims.user_id, &req.name).await;
    if let Err(e) = new_portfolio {
        return ApiResponse::from(e);
    }
    let new_portfolio = new_portfolio.unwrap();
    info!("New portfolio id {}", new_portfolio.id);
    ApiResponse::success(IdResponse {
        id: new_portfolio.id.to_string(),
    })
}

#[utoipa::path(
    post,
    path = "/api/portfolios/{portfolio_id}/assets",
    responses(
        (status = 200, description = "Success", body = ApiResponse<IdResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_portfolio_asset(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreatePortfolioAssetRequest>,
) -> ApiResponse<GeneralResponse> {
    let pfl_repo = PortfolioRepo::new(state.pool.clone());
    let pfl_row = pfl_repo.get_one_by_id(req.portfolio_id).await;
    if let Err(e) = pfl_row {
        return ApiResponse::from(e);
    }
    let pfl_row = pfl_row.unwrap();
    if pfl_row.is_none() {
        return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio not found".to_string());
    }
    let pfl_row = pfl_row.unwrap();
    if pfl_row.owner_id != claims.user_id {
        return ApiResponse::error(
            StatusCode::FORBIDDEN,
            "You are not the owner of this portfolio".to_string(),
        );
    }
    let asset_repo = AssetRepo::new(state.pool.clone());
    let asset = asset_repo.get_one_by_id(&req.asset_id).await;
    if let Err(e) = asset {
        return ApiResponse::from(e);
    }
    let asset = asset.unwrap();
    if asset.is_none() {
        return ApiResponse::error(StatusCode::NOT_FOUND, "Asset not found".to_string());
    }

    let pa_repo = PortfolioAssetRepo::new(state.pool.clone());
    let pa_row = pa_repo.create(req.portfolio_id, &req.asset_id).await;
    if let Err(e) = pa_row {
        return ApiResponse::from(e);
    }
    return ApiResponse::<GeneralResponse>::success_general_response();
}

#[utoipa::path(
    get,
    path = "/api/portfolios/{id}",
    responses(
        (status = 200, description = "Success", body = ApiResponse<PortfolioResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_portfolio_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResponse<PortfolioResponse> {
    info!("Get portfolio with id {}", id);
    let pfl_repo = PortfolioRepo::new(state.pool.clone());
    let pfl_row = pfl_repo.get_one_by_id(id.parse().unwrap()).await;
    match pfl_row {
        Err(e) => {
            return ApiResponse::from(e);
        }
        Ok(None) => {
            return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio not found".to_string());
        }
        _ => {}
    }
    let pfl_row = pfl_row.unwrap().unwrap();
    let pa_repo = PortfolioAssetRepo::new(state.pool.clone());
    let pa_rows = pa_repo.get_multi_by_portfolio_id(pfl_row.id).await;
    if let Err(e) = pa_rows {
        return ApiResponse::from(e);
    }
    let pa_rows = pa_rows.unwrap();
    let asset_ids: Vec<String> = pa_rows.iter().map(|a| a.asset_id.clone()).collect();
    let asset_repo = AssetRepo::new(state.pool.clone());
    let result = asset_repo.get_multi_by_ids(&asset_ids).await;
    if let Err(e) = result {
        return ApiResponse::from(e);
    }
    let asset_rows = result.unwrap();
    ApiResponse::success(PortfolioResponse {
        id: pfl_row.id.to_string(),
        name: pfl_row.name,
        assets: asset_rows
            .into_iter()
            .map(|row| AssetResponse::from_row(row))
            .collect(),
    })
}

#[utoipa::path(
    get,
    path = "/api/portfolios",
    responses(
        (status = 200, description = "Success", body = ApiResponse<BriefPortfolioListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_my_portfolios(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> ApiResponse<BriefPortfolioListResponse> {
    let portfolio_repo = PortfolioRepo::new(state.pool.clone());
    let portfolios = portfolio_repo.get_multi_by_owner_id(claims.user_id).await;
    if let Err(e) = portfolios {
        return ApiResponse::from(e);
    }
    let portfolios = portfolios.unwrap();
    ApiResponse::success(BriefPortfolioListResponse {
        items: portfolios
            .into_iter()
            .map(|portfolio| BriefPortfolioResponse::from_row(portfolio))
            .collect(),
    })
}
