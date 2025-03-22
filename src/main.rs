mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod state;
mod utils;
use anyhow::Result;
use axum::Router;
use config::load_config;
use db::postgres::init_pg_pool;
use std::sync::Arc;
use tokio::signal;
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Generate API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        // List your handler functions here
        handlers::health::health_check,
        handlers::auth::login_with_password,
        handlers::users::get_user_by_id,
    ),
    components(
        // List your schema components here
        schemas(
            models::auth::LoginWithPasswordRequest,
            models::auth::AuthResponse,
            models::user::UserResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "app=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting application");

    // load config
    let settings = load_config()?;
    info!("Config loaded successfully");

    // init pg pool
    let pg_pool = init_pg_pool(&settings.database_url).await?;
    // Create app state
    let state = Arc::new(
        state::AppStateInner::new(pg_pool, settings.secret_key, settings.redis_url)
            .await
            .unwrap(),
    );
    state.health_check().await?;
    info!("Health check state passed");

    // Create separate routers
    let public_routes = Router::new()
        .nest("/health", routes::health::create_router())
        .nest("/auth", routes::auth::create_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let protected_routes = Router::new()
        .nest("/api", routes::api::create_router())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::require_authentication,
        ));

    // Combine routers
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state.clone())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );
    info!("Router configured");

    let addr = format!("0.0.0.0:{}", settings.server_port);
    info!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    // run server
    info!("Server starting...");
    axum::serve(listener, app).await?;

    // Handle shutdown signal
    tokio::spawn(async move {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to install signal handler")
                .recv()
                .await;
        };
        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        // Initiate graceful shutdown
        println!("Shutting down gracefully...");
        state.shutdown().await.expect("Shutdown state failed");
    });
    Ok(())
}
