// src/routes/mod.rs – Gabungkan semua router

use axum::Router;
use crate::AppState;

pub mod auth;
pub mod reports;
pub mod admin;
pub mod categories;

pub fn all_routes(rate_limit_burst: u32) -> Router<AppState> {
    Router::new()
        .merge(auth::router(rate_limit_burst))
        .merge(reports::router())
        .merge(admin::router())
        .merge(categories::router())
}

