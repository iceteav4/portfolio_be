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
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
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
    info!("Database connection established");
    // Create app state
    let state = Arc::new(
        state::AppStateInner::new(pg_pool, settings.secret_key, settings.redis_url)
            .await
            .unwrap(),
    );
    // Create separate routers
    let public_routes = Router::new()
        .nest("/health", routes::health::create_router())
        .nest("/auth", routes::auth::create_router());

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
        .with_state(state)
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
    Ok(())
}
