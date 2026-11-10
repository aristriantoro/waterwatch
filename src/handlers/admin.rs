// src/handlers/admin.rs – Handler fitur admin

use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    middleware::role_guard::AdminClaims,
    models::report::{
        AdminReportQuery, PaginatedResponse, ReportDetail, ReportSummaryAdmin,
        UpdateStatusRequest, UpdateStatusResponse,
    },
    services::admin as admin_svc,
    AppState,
};

/// GET /api/v1/admin/dashboard
pub async fn dashboard(
    State(state): State<AppState>,
    AdminClaims(_claims): AdminClaims,
) -> Result<Json<admin_svc::DashboardResponse>, AppError> {
    let resp = admin_svc::get_dashboard(&state.pool).await?;
    Ok(Json(resp))
}

/// GET /api/v1/admin/reports
pub async fn list_reports(
    State(state): State<AppState>,
    AdminClaims(_claims): AdminClaims,
    Query(q): Query<AdminReportQuery>,
) -> Result<Json<PaginatedResponse<ReportSummaryAdmin>>, AppError> {
    let resp = admin_svc::list_reports(&state.pool, &q).await?;
    Ok(Json(resp))
}

/// GET /api/v1/admin/reports/{id}  (detail laporan untuk admin)
pub async fn get_report_detail(
    State(state): State<AppState>,
    AdminClaims(claims): AdminClaims,
    Path(id): Path<Uuid>,
) -> Result<Json<ReportDetail>, AppError> {
    let detail = crate::services::reports::get_detail_for_user(
        &state.pool, id, claims.sub, true,
    ).await?;
    Ok(Json(detail))
}

/// PATCH /api/v1/admin/reports/{id}/status
pub async fn update_status(
    State(state): State<AppState>,
    AdminClaims(claims): AdminClaims,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<UpdateStatusResponse>, AppError> {
    let resp = admin_svc::update_status(&state.pool, id, req, claims.sub).await?;
    Ok(Json(resp))
}

