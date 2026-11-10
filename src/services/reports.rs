// src/services/reports.rs – Logika bisnis laporan warga

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::{AppError, FieldError},
    models::report::{
        CreateReportInput, CreateReportResponse, PaginatedResponse,
        ReportDetail, ReportQuery, ReportSummary,
    },
    repositories::{category as cat_repo, report as report_repo},
};

/// Validasi dan simpan foto ke filesystem
pub async fn save_photo(
    data: Vec<u8>,
    upload_dir: &str,
    max_bytes: u64,
) -> Result<String, AppError> {
    if data.len() as u64 > max_bytes {
        return Err(AppError::PayloadTooLarge);
    }
    let kind = infer::get(&data).ok_or(AppError::UnsupportedMediaType)?;
    let ext = match kind.mime_type() {
        "image/jpeg" => "jpg",
        "image/png"  => "png",
        _            => return Err(AppError::UnsupportedMediaType),
    };
    let filename = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let path = std::path::Path::new(upload_dir).join(&filename);
    tokio::fs::write(&path, &data)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Gagal simpan foto: {e}")))?;
    Ok(filename)
}

/// Buat laporan baru
pub async fn create_report(
    pool: &PgPool,
    user_id: Uuid,
    input: CreateReportInput,
    upload_dir: &str,
    max_upload_bytes: u64,
) -> Result<CreateReportResponse, AppError> {
    // Validasi
    let mut errs: Vec<FieldError> = vec![];

    if input.deskripsi.len() < 10 || input.deskripsi.len() > 1000 {
        errs.push(FieldError::with_message("deskripsi", "Deskripsi harus 10–1000 karakter"));
    }
    if input.lokasi.trim().is_empty() {
        errs.push(FieldError::with_message("lokasi", "Lokasi wajib diisi"));
    }
    if let Some(lat) = input.latitude {
        if !(-90.0..=90.0).contains(&lat) {
            errs.push(FieldError::with_message("latitude", "Latitude harus antara -90 dan 90"));
        }
    }
    if let Some(lng) = input.longitude {
        if !(-180.0..=180.0).contains(&lng) {
            errs.push(FieldError::with_message("longitude", "Longitude harus antara -180 dan 180"));
        }
    }
    // Validasi kategori ada
    if cat_repo::find_by_id(pool, input.kategori_id).await?.is_none() {
        errs.push(FieldError::with_message("kategori_id", "Kategori tidak ditemukan"));
    }
    if !errs.is_empty() {
        return Err(AppError::Validation(errs));
    }

    // Simpan foto jika ada
    let foto_filename = if let Some(data) = input.foto_data {
        Some(save_photo(data, upload_dir, max_upload_bytes).await?)
    } else {
        None
    };

    report_repo::create_report(
        pool,
        user_id,
        input.kategori_id,
        &input.deskripsi,
        &input.lokasi,
        input.latitude,
        input.longitude,
        input.wilayah.as_deref(),
        foto_filename.as_deref(),
    )
    .await
}

/// Riwayat laporan milik warga
pub async fn get_mine(
    pool: &PgPool,
    user_id: Uuid,
    q: &ReportQuery,
) -> Result<PaginatedResponse<ReportSummary>, AppError> {
    let (data, total) = report_repo::find_mine(pool, user_id, q).await?;
    Ok(PaginatedResponse::new(data, q.page, q.per_page, total))
}

/// Detail laporan – cek kepemilikan (warga hanya milik sendiri → 404)
pub async fn get_detail_for_user(
    pool: &PgPool,
    report_id: Uuid,
    user_id: Uuid,
    is_admin: bool,
) -> Result<ReportDetail, AppError> {
    if !is_admin {
        let owner = report_repo::get_report_owner(pool, report_id).await?;
        match owner {
            Some(oid) if oid == user_id => {}
            _ => return Err(AppError::NotFound), // 404, jangan bocorkan keberadaan
        }
    }
    report_repo::find_detail_by_id(pool, report_id)
        .await?
        .ok_or(AppError::NotFound)
}

