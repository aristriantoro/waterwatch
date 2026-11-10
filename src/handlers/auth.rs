// src/handlers/auth.rs – Handler tipis untuk endpoint autentikasi

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    errors::AppError,
    models::user::{Claims, LoginRequest, LoginResponse, RegisterRequest, UserResponse},
    services::auth as auth_svc,
    AppState,
};

/// POST /api/v1/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = auth_svc::register(&state.pool, req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

/// POST /api/v1/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let resp = auth_svc::login(
        &state.pool,
        req,
        &state.config.jwt_secret,
        state.config.jwt_expiry_seconds,
    )
    .await?;
    Ok(Json(resp))
}

/// GET /api/v1/auth/me
pub async fn me(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<UserResponse>, AppError> {
    let user = auth_svc::get_me(&state.pool, claims.sub).await?;
    Ok(Json(user))
}

