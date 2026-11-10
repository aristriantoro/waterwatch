// src/main.rs – Bootstrap server WaterWatch

mod config;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod handlers;

use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowHeaders, AllowOrigin, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use axum::http::header;

pub use config::Config;

/// State yang di-share ke semua handler
#[derive(Clone)]
pub struct AppState {
    pub pool:   PgPool,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Muat .env
    dotenvy::dotenv().ok();

    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "waterwatch=info,tower_http=info".into()),
        )
        .init();

    // Muat konfigurasi
    let config = Config::from_env()?;
    tracing::info!("Starting WaterWatch on port {}", config.server_port);

    // Koneksi database
    let pool = PgPool::connect(&config.database_url).await?;
    tracing::info!("Database connected");

    // Pastikan folder uploads ada
    tokio::fs::create_dir_all(&config.upload_dir).await?;

    // CORS
    let allowed_origins: Vec<HeaderValue> = config
        .allowed_origins
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::OPTIONS])
        .allow_headers(AllowHeaders::mirror_request());

    // Header keamanan
    let security_headers = tower::ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ));

    let state = AppState { pool, config: config.clone() };

    // Health check handler
    let health = || async {
        Json(serde_json::json!({
            "status": "ok",
            "version": env!("CARGO_PKG_VERSION")
        }))
    };

    let app = Router::new()
        .route("/health", get(health))
        .merge(routes::all_routes(config.rate_limit_burst))
        .with_state(state)
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(security_headers);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    // Gunakan into_make_service_with_connect_info agar tower_governor bisa baca IP klien
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
