use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::info;

use crate::{
    db::repositories::portfolio::PortfolioRepo,
    models::{
        domain::{auth::Claims, portfolio::CreatePortfolio},
        dto::{
            api_response::ApiResponse,
            id_response::IdResponse,
            portfolio::{
                BriefPortfolioListResponse, BriefPortfolioResponse, CreatePortfolioRequest,
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
    let new_portfolio = portfolio_repo
        .create(CreatePortfolio {
            owner_id: claims.user_id,
            name: req.name,
        })
        .await;
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
        (status = 200, description = "Success", body = ApiResponse<BriefPortfolioResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_portfolio_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResponse<BriefPortfolioResponse> {
    info!("Get portfolio with id {}", id);
    let portfolio_repo = PortfolioRepo::new(state.pool.clone());
    let portfolio = portfolio_repo.get_by_id(id.parse().unwrap()).await;
    if let Err(e) = portfolio {
        return ApiResponse::from(e);
    }
    let portfolio = portfolio.unwrap();
    if portfolio.is_none() {
        return ApiResponse::error(StatusCode::NOT_FOUND, "Portfolio not found".to_string());
    }
    let portfolio = portfolio.unwrap();
    ApiResponse::success(BriefPortfolioResponse::from_entity(portfolio))
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
