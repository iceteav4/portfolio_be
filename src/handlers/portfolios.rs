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
            portfolio::{
                BriefPortfolioListResponse, BriefPortfolioResponse, CreatePortfolioAssetRequest,
                CreatePortfolioRequest, PortfolioAssetResponse, PortfolioResponse,
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
    Path(portfolio_id): Path<String>,
    Json(req): Json<CreatePortfolioAssetRequest>,
) -> ApiResponse<GeneralResponse> {
    let pfl_repo = PortfolioRepo::new(state.pool.clone());
    let pfl_id: i64 = portfolio_id.parse().unwrap();
    let pfl_rs = pfl_repo.get_one_by_id(pfl_id).await;
    let pfl_row = match pfl_rs {
        Err(e) => return ApiResponse::from(e),
        Ok(None) => {
            return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio not found".to_string());
        }
        Ok(Some(pfl_row)) => pfl_row,
    };
    if pfl_row.owner_id != claims.user_id {
        return ApiResponse::error(
            StatusCode::FORBIDDEN,
            "You are not the owner of this portfolio".to_string(),
        );
    }
    let asset_repo = AssetRepo::new(state.pool.clone());
    let asset_rs = asset_repo.get_one_by_id(&req.asset_id).await;
    match asset_rs {
        Err(e) => return ApiResponse::from(e),
        Ok(None) => {
            return ApiResponse::error(StatusCode::NOT_FOUND, "Asset not found".to_string());
        }
        Ok(Some(asset_row)) => asset_row,
    };

    let pa_repo = PortfolioAssetRepo::new(state.pool.clone());
    let pa_rs = pa_repo
        .get_one_by_portfolio_id_and_asset_id(pfl_id, &req.asset_id)
        .await;
    match pa_rs {
        Err(e) => return ApiResponse::from(e),
        Ok(Some(_)) => {
            return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio asset already exists");
        }
        _ => (),
    };
    let pa_row = pa_repo.create(pfl_id, &req.asset_id).await;
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
    let pfl_rs = pfl_repo.get_one_by_id(id.parse().unwrap()).await;
    let pfl_row = match pfl_rs {
        Err(e) => return ApiResponse::from(e),
        Ok(None) => {
            return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio not found");
        }
        Ok(Some(row)) => row,
    };
    let pa_repo = PortfolioAssetRepo::new(state.pool.clone());
    let pa_rs = pa_repo.get_multi_by_portfolio_id(pfl_row.id).await;
    let pa_rows = match pa_rs {
        Err(e) => return ApiResponse::from(e),
        Ok(rows) => rows,
    };
    let asset_ids: Vec<String> = pa_rows.iter().map(|a| a.asset_id.clone()).collect();
    let asset_repo = AssetRepo::new(state.pool.clone());
    let assets_rs = asset_repo.get_multi_by_ids(&asset_ids).await;
    let asset_rows = match assets_rs {
        Err(e) => return ApiResponse::from(e),
        Ok(rows) => rows,
    };
    ApiResponse::success(PortfolioResponse {
        id: pfl_row.id.to_string(),
        name: pfl_row.name,
        assets: asset_rows
            .into_iter()
            .map(|asset| PortfolioAssetResponse::from_db_rows(asset, Vec::new()))
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
