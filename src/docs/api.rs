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

        // Portfolio endpoints
        handlers::portfolios::create_portfolio,
        handlers::portfolios::get_portfolio_by_id,
        handlers::portfolios::get_my_portfolios,

        // Import endpoints
        handlers::import::coingecko::import_portfolio_file,
        handlers::import::coingecko::get_coin_data_by_id,
    ),
    components(
        // List your schema components here
        schemas(
            dto::auth::LoginWithPasswordRequest,
            dto::auth::SignUpWithPasswordRequest,
            dto::auth::AuthResponse,
            dto::portfolio::CreatePortfolioRequest,
            dto::portfolio::PortfolioResponse,
            dto::portfolio::BriefPortfolioListResponse,
            dto::user::UserResponse,
            dto::user::UserMeResponse,
            dto::id_response::IdResponse,
            dto::api_response::GeneralResponse,
            dto::health::HealthResponse,
            dto::coingecko::CoinDataResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User endpoints"),
        (name = "portfolios", description = "Portfolio endpoints"),
        (name = "imports", description = "Import endpoints")
    )
)]
pub struct ApiDoc;
