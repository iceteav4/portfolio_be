use crate::state::AppState;
use axum::Router;
pub mod assets;
pub mod imports;
pub mod portfolios;
pub mod transactions;
pub mod users;

/// Creates the protected API router.
/// 
/// All routes under this router require JWT authentication.
/// Handlers in this router can access the authenticated user via:
/// `Extension(claims): Extension<Claims>`
pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/users", users::create_router())
        .nest("/imports", imports::create_router())
        .nest("/portfolios", portfolios::create_router())
        .nest("/assets", assets::create_router())
}
