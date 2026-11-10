// src/routes/auth.rs – Route autentikasi dengan rate limit pada /login

use axum::{routing::{get, post}, Router};
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
};
use crate::{handlers::auth as h, AppState};

/// `burst_size`: jumlah request login yang diizinkan sebelum di-throttle.
/// Default produksi = 5, test bisa set ke 1000 via RATE_LIMIT_BURST env.
pub fn router(burst_size: u32) -> Router<AppState> {
    // Box::leak memberi lifetime 'static yang dibutuhkan oleh Layer
    let login_governor_conf: &'static _ = Box::leak(Box::new(
        GovernorConfigBuilder::default()
            .per_second(60)
            .burst_size(burst_size)
            .finish()
            .expect("GovernorConfig build failed"),
    ));

    let login_router = Router::new()
        .route("/api/v1/auth/login", post(h::login))
        .layer(GovernorLayer { config: login_governor_conf });

    Router::new()
        .route("/api/v1/auth/register", post(h::register))
        .route("/api/v1/auth/me", get(h::me))
        .merge(login_router)
}
