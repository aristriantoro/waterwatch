// src/models/report.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

/// Status laporan – harus cocok dengan ENUM di PostgreSQL
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "report_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ReportStatus {
    Menunggu,
    Diproses,
    Selesai,
}

impl std::fmt::Display for ReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportStatus::Menunggu => write!(f, "menunggu"),
            ReportStatus::Diproses => write!(f, "diproses"),
            ReportStatus::Selesai  => write!(f, "selesai"),
        }
    }
}

/// Validasi transisi status (aturan bisnis inti)
/// menunggu → diproses → selesai (selesai = final)
pub fn is_valid_transition(from: &ReportStatus, to: &ReportStatus) -> bool {
    matches!(
        (from, to),
        (ReportStatus::Menunggu, ReportStatus::Diproses)
            | (ReportStatus::Diproses, ReportStatus::Selesai)
    )
}

/// Baris di tabel `reports`
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Report {
    pub id:          Uuid,
    pub nomor:       String,
    pub user_id:     Uuid,
    pub category_id: i16,
    pub deskripsi:   String,
    pub lokasi:      String,
    pub latitude:    Option<f64>,
    pub longitude:   Option<f64>,
    pub wilayah:     Option<String>,
    pub foto:        Option<String>,
    pub status:      ReportStatus,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
}

/// Ringkasan laporan (untuk list/riwayat)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ReportSummary {
    pub id:          Uuid,
    pub nomor:       String,
    pub kategori:    String,
    pub deskripsi:   String,
    pub lokasi:      String,
    pub status:      ReportStatus,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
}

/// Ringkasan laporan versi admin (ada user_nama)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ReportSummaryAdmin {
    pub id:          Uuid,
    pub nomor:       String,
    pub user_nama:   String,
    pub kategori:    String,
    pub deskripsi:   String,
    pub lokasi:      String,
    pub status:      ReportStatus,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>,
}

/// Detail laporan lengkap (dengan riwayat)
#[derive(Debug, Serialize)]
pub struct ReportDetail {
    pub id:         Uuid,
    pub nomor:      String,
    pub user_nama:  Option<String>,  // ada saat admin mengakses
    pub kategori:   String,
    pub deskripsi:  String,
    pub lokasi:     String,
    pub latitude:   Option<f64>,
    pub longitude:  Option<f64>,
    pub wilayah:    Option<String>,
    pub foto:       Option<String>,
    pub status:     ReportStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub riwayat:    Vec<crate::models::report_update::ReportUpdateResponse>,
}

/// DTO: request membuat laporan (dari multipart)
#[derive(Debug)]
pub struct CreateReportInput {
    pub kategori_id: i16,
    pub deskripsi:   String,
    pub lokasi:      String,
    pub latitude:    Option<f64>,
    pub longitude:   Option<f64>,
    pub wilayah:     Option<String>,
    pub foto_data:   Option<Vec<u8>>,   // raw bytes foto (divalidasi di service)
}

/// DTO: request ubah status (dari admin)
#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status:      ReportStatus,
    pub keterangan:  String,
}

/// Query parameter untuk GET /reports/mine
#[derive(Debug, Deserialize)]
pub struct ReportQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
    pub status: Option<ReportStatus>,
}

/// Query parameter untuk GET /admin/reports
#[derive(Debug, Deserialize)]
pub struct AdminReportQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
    pub status:          Option<ReportStatus>,
    pub kategori_id:     Option<i16>,
    pub tanggal_dari:    Option<chrono::NaiveDate>,
    pub tanggal_sampai:  Option<chrono::NaiveDate>,
    pub q:               Option<String>,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 10 }

/// Respons paginasi generik
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page:        u32,
    pub per_page:    u32,
    pub total:       i64,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: u32, per_page: u32, total: i64) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
        Self {
            data,
            meta: PaginationMeta { page, per_page, total, total_pages },
        }
    }
}

/// Respons singkat setelah membuat laporan
#[derive(Debug, Serialize)]
pub struct CreateReportResponse {
    pub id:         Uuid,
    pub nomor:      String,
    pub status:     ReportStatus,
    pub created_at: DateTime<Utc>,
}

/// Respons singkat setelah update status
#[derive(Debug, Serialize)]
pub struct UpdateStatusResponse {
    pub id:         Uuid,
    pub status:     ReportStatus,
    pub updated_at: DateTime<Utc>,
}

