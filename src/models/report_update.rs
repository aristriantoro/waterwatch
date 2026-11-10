// src/models/report_update.rs

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::models::report::ReportStatus;

/// Baris di tabel `report_updates`
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ReportUpdate {
    pub id:          i64,
    pub report_id:   Uuid,
    pub status:      ReportStatus,
    pub keterangan:  Option<String>,
    pub updated_by:  Option<Uuid>,
    pub updated_at:  DateTime<Utc>,
}

/// Versi serialisasi (dengan nama admin, bukan UUID)
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ReportUpdateResponse {
    pub status:          ReportStatus,
    pub keterangan:      Option<String>,
    pub updated_by_nama: Option<String>,
    pub updated_at:      DateTime<Utc>,
}

