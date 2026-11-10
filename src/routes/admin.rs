// src/routes/admin.rs

use axum::{
    routing::{get, patch},
    Router,
};
use crate::{handlers::admin as h, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/admin/dashboard",             get(h::dashboard))
        .route("/api/v1/admin/reports",               get(h::list_reports))
        .route("/api/v1/admin/reports/:id",           get(h::get_report_detail))
        .route("/api/v1/admin/reports/:id/status",    patch(h::update_status))
}

