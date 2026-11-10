// src/middleware/role_guard.rs – Extractor Claims khusus role admin

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use crate::{errors::AppError, models::user::Claims, AppState};

/// Extractor: `AdminClaims` – hanya lolos jika role == "admin"
pub struct AdminClaims(pub Claims);

#[axum::async_trait]
impl FromRequestParts<AppState> for AdminClaims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;
        if claims.role != "admin" {
            return Err(AppError::Forbidden);
        }
        Ok(AdminClaims(claims))
    }
}

