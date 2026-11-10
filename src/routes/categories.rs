// src/routes/categories.rs

use axum::{routing::get, Router};
use crate::{handlers::categories as h, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/categories", get(h::list_categories))
}

