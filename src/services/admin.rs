// src/services/admin.rs – Logika bisnis fitur admin

use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::report::{
        AdminReportQuery, PaginatedResponse, ReportSummaryAdmin,
        UpdateStatusRequest, UpdateStatusResponse, is_valid_transition,
    },
    repositories::report as report_repo,
};

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub total:          i64,
    pub per_status:     StatusCount,
    pub hari_ini:       i64,
    pub per_kategori:   Vec<KategoriStat>,
    pub laporan_terbaru: Vec<ReportSummaryAdmin>,
}

#[derive(Debug, Serialize)]
pub struct StatusCount {
    pub menunggu: i64,
    pub diproses: i64,
    pub selesai:  i64,
}

#[derive(Debug, Serialize)]
pub struct KategoriStat {
    pub kategori: String,
    pub jumlah:   i64,
}

pub async fn get_dashboard(pool: &PgPool) -> Result<DashboardResponse, AppError> {
    let stats = report_repo::get_dashboard_stats(pool).await?;
    let cat_stats = report_repo::get_category_stats(pool).await?;
    let latest = report_repo::find_latest(pool, 10).await?;

    Ok(DashboardResponse {
        total:    stats.total,
        per_status: StatusCount {
            menunggu: stats.menunggu,
            diproses: stats.diproses,
            selesai:  stats.selesai,
        },
        hari_ini: stats.hari_ini,
        per_kategori: cat_stats
            .into_iter()
            .map(|s| KategoriStat { kategori: s.kategori, jumlah: s.jumlah })
            .collect(),
        laporan_terbaru: latest,
    })
}

pub async fn list_reports(
    pool: &PgPool,
    q: &AdminReportQuery,
) -> Result<PaginatedResponse<ReportSummaryAdmin>, AppError> {
    let (data, total) = report_repo::find_all_admin(pool, q).await?;
    Ok(PaginatedResponse::new(data, q.page, q.per_page, total))
}

/// Ubah status laporan dengan validasi transisi
pub async fn update_status(
    pool: &PgPool,
    report_id: Uuid,
    req: UpdateStatusRequest,
    admin_id: Uuid,
) -> Result<UpdateStatusResponse, AppError> {
    // Keterangan wajib
    if req.keterangan.trim().is_empty() {
        return Err(AppError::Validation(vec![
            crate::errors::FieldError::with_message(
                "keterangan",
                "Keterangan wajib diisi saat memperbarui status",
            ),
        ]));
    }

    // Ambil status saat ini
    let detail = report_repo::find_detail_by_id(pool, report_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Validasi transisi
    if !is_valid_transition(&detail.status, &req.status) {
        return Err(AppError::InvalidTransition(format!(
            "Tidak bisa mengubah status dari '{}' ke '{}'",
            detail.status, req.status
        )));
    }

    report_repo::update_status(pool, report_id, req.status, &req.keterangan, admin_id).await
}

