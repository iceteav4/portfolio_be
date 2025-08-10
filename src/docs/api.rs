use utoipa::OpenApi;

use crate::handlers;
use crate::models::dto;

// Generate API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        // Health endpoints
        handlers::health::health_check,

        // Auth endpoints
        handlers::auth::login_with_password,
        handlers::auth::signup,

        // User endpoints
        handlers::users::get_user_by_id,
        handlers::users::get_user_me,

        // Asset endpoints
        handlers::assets::get_all_assets,
        handlers::assets::create_asset,

        // Portfolio endpoints
        handlers::portfolios::create_portfolio,
        handlers::portfolios::create_portfolio_asset,
        handlers::portfolios::get_portfolio_by_id,
        handlers::portfolios::get_my_portfolios,

        // Import endpoints
        handlers::import::coingecko::import_portfolio_file,
        handlers::import::coingecko::get_coin_data_by_id,
    ),
    components(
        // List your schema components here
        schemas(
            dto::asset::AssetResponse,
            dto::asset::AssetListResponse,
            dto::auth::LoginWithPasswordRequest,
            dto::auth::SignUpWithPasswordRequest,
            dto::auth::AuthResponse,
            dto::portfolio::CreatePortfolioRequest,
            dto::portfolio::PortfolioResponse,
            dto::portfolio::BriefPortfolioListResponse,
            dto::portfolio_asset::CreatePortfolioAssetRequest,
            dto::portfolio_asset::PortfolioAssetResponse,
            dto::user::UserResponse,
            dto::user::UserMeResponse,
            dto::api_response::GeneralResponse,
            dto::api_response::IdResponse,
            dto::health::HealthResponse,
            dto::coingecko::CoinDataResponse,
            dto::transaction::TransactionResponse,
            dto::transaction::TransactionListResponse
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User endpoints"),
        (name = "portfolios", description = "Portfolio endpoints"),
        (name = "transactions", description = "Transaction endpoints"),
        (name = "assets", description = "Asset endpoints"),
        (name = "imports", description = "Import endpoints")
    )
)]
pub struct ApiDoc;
