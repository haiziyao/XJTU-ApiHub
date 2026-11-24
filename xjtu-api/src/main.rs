mod auth;
mod common;
mod config;
mod error;

use std::sync::Arc;
use std::time::Duration;

use axum::routing::{get, post};
use axum::Router;
use sqlx::mysql::MySqlPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::common::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = config::Config::from_env()?;
    let bind_addr = config.bind_addr.clone();

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&config.database_url)
        .await?;

    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()?;

    let state = AppState {
        pool,
        config: Arc::new(config),
        http,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/auth/register/lookup", post(auth::routes::lookup))
        .route("/api/auth/register", post(auth::routes::register))
        .route("/api/auth/login", post(auth::routes::login))
        .route(
            "/api-test/auth/register/lookup",
            post(auth::routes::lookup_test),
        )
        .route("/api-test/auth/register", post(auth::routes::register_test))
        .route("/api-test/auth/login", post(auth::routes::login_test))
        .route("/api/me", get(auth::routes::me))
        .route("/api/index", get(auth::routes::index))
        .route("/api-test/index", get(auth::routes::index_test))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    tracing::info!("listening on {}", bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> &'static str {
    "ok"
}
