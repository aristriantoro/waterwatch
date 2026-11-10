// src/handlers/categories.rs

use axum::{extract::State, Json};
use crate::{errors::AppError, repositories::category as cat_repo, AppState};

/// GET /api/v1/categories
pub async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let cats = cat_repo::find_all(&state.pool).await?;
    Ok(Json(serde_json::json!({ "data": cats })))
}

