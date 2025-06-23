use anyhow::Result;
use axum::{
    Router,
    http::{HeaderValue, Method, header},
};
use config::load_config;
use std::sync::Arc;
use tokio::signal;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::{Level, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod biz;
mod clients;
mod config;
mod db;
mod docs;
mod handlers;
mod middleware;
mod models;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // load config
    let settings = load_config()?;

    // Initialize tracing with config from settings
    let log_filter = format!("{}={}", env!("CARGO_PKG_NAME"), settings.logging.level);
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_filter))
        .with(tracing_subscriber::fmt::layer().with_ansi(true))
        .init();
    info!(
        "Starting application with log level: {}",
        settings.logging.level
    );

    // Create app state
    let state = Arc::new(state::AppStateInner::new(&settings).await.unwrap());
    state.health_check().await?;
    info!("Health check state passed");

    // Create cors layer
    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::list(
            settings
                .server
                .allow_origins
                .iter()
                .map(|s| s.parse::<HeaderValue>().unwrap()),
        ))
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers(vec![
            header::ACCEPT,
            header::CONTENT_TYPE,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
            header::HeaderName::from_static("x-request-id"),
        ]);

    // Create separate routers
    let public_routes = Router::new()
        .nest("/health", routes::health::create_router())
        .nest("/auth", routes::auth::create_router())
        .layer(cors_layer)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", docs::api::ApiDoc::openapi()));

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
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(axum::middleware::from_fn(
            middleware::trace::trace_request_id,
        ));
    info!("Router configured");

    let addr = format!("0.0.0.0:{}", settings.server.port);
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
