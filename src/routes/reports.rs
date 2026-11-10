// src/routes/reports.rs

use axum::{
    routing::{get, post},
    Router,
};
use crate::{handlers::reports as h, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/reports",          post(h::create_report))
        .route("/api/v1/reports/mine",     get(h::list_mine))
        .route("/api/v1/reports/:id",      get(h::get_detail))
        .route("/api/v1/reports/:id/foto", get(h::get_foto))
}

