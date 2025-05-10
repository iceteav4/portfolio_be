use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::info;

use crate::{
    biz::portfolio::PortfolioBiz,
    db::repositories::{asset::AssetRepo, portfolio::PortfolioRepo},
    models::{
        domain::auth::Claims,
        dto::{
            api_response::ApiResponse,
            id_response::IdResponse,
            portfolio::{
                BriefPortfolioListResponse, BriefPortfolioResponse, CreatePortfolioRequest,
                PortfolioResponse,
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
    let new_portfolio = portfolio_repo.create(claims.user_id, &req.name).await;
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
    let portfolio_biz = PortfolioBiz::new(state.pool.clone());
    let portfolio = portfolio_biz.get_by_portfolio_id(id.parse().unwrap()).await;
    if let Err(e) = portfolio {
        return ApiResponse::from(e);
    }
    let portfolio = portfolio.unwrap();
    if portfolio.is_none() {
        return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio not found".to_string());
    }
    let portfolio = portfolio.unwrap();
    let asset_ids: Vec<String> = portfolio
        .assets
        .iter()
        .map(|a| a.asset_id.clone())
        .collect();
    let asset_repo = AssetRepo::new(state.pool.clone());
    let result = asset_repo.get_multi_by_ids(&asset_ids).await;
    if let Err(e) = result {
        return ApiResponse::from(e);
    }
    let asset_rows = result.unwrap();
    ApiResponse::success(PortfolioResponse::from_entity(portfolio, asset_rows))
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
            .map(|portfolio| BriefPortfolioResponse::from_entity(portfolio))
            .collect(),
    })
}
