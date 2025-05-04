use utoipa::OpenApi;

use crate::handlers;
use crate::models::dto;

// Generate API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        // List your handler functions here
        handlers::import::coingecko::import_portfolio_file,
        handlers::health::health_check,
        handlers::auth::login_with_password,
        handlers::auth::signup,
        handlers::users::get_user_by_id,
        handlers::users::get_user_me,
        handlers::portfolios::create_portfolio,
        handlers::portfolios::get_portfolio_by_id,
    ),
    components(
        // List your schema components here
        schemas(
            dto::auth::LoginWithPasswordRequest,
            dto::auth::SignUpWithPasswordRequest,
            dto::auth::AuthResponse,
            dto::portfolio::CreatePortfolioRequest,
            dto::portfolio::BriefPortfolioResponse,
            dto::user::UserResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User endpoints"),
        (name = "portfolios", description = "Portfolio endpoints")
    )
)]
pub struct ApiDoc;
