// src/handlers/reports.rs – Handler laporan warga

use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        report::{
            CreateReportResponse, PaginatedResponse, ReportDetail, ReportQuery,
            ReportSummary,
        },
        user::Claims,
    },
    services::reports as report_svc,
    AppState,
};

/// POST /api/v1/reports (multipart)
pub async fn create_report(
    State(state): State<AppState>,
    claims: Claims,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<CreateReportResponse>), AppError> {
    // Parse multipart fields
    let mut kategori_id: Option<i16> = None;
    let mut deskripsi:   Option<String> = None;
    let mut lokasi:      Option<String> = None;
    let mut latitude:    Option<f64> = None;
    let mut longitude:   Option<f64> = None;
    let mut wilayah:     Option<String> = None;
    let mut foto_data:   Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Multipart error: {e}")))?
    {
        match field.name() {
            Some("kategori_id") => {
                let v = field.text().await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("{e}")))?;
                kategori_id = v.trim().parse::<i16>().ok();
            }
            Some("deskripsi") => {
                deskripsi = Some(field.text().await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("{e}")))?);
            }
            Some("lokasi") => {
                lokasi = Some(field.text().await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("{e}")))?);
            }
            Some("latitude") => {
                let v = field.text().await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("{e}")))?;
                latitude = v.trim().parse::<f64>().ok();
            }
            Some("longitude") => {
                let v = field.text().await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("{e}")))?;
                longitude = v.trim().parse::<f64>().ok();
            }
            Some("wilayah") => {
                let v = field.text().await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("{e}")))?;
                if !v.trim().is_empty() { wilayah = Some(v); }
            }
            Some("foto") => {
                let bytes = field.bytes().await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("{e}")))?;
                if !bytes.is_empty() {
                    foto_data = Some(bytes.to_vec());
                }
            }
            _ => {}
        }
    }

    // Validasi field wajib ada sebelum dikirim ke service
    let mut errs = vec![];
    if kategori_id.is_none() {
        errs.push(crate::errors::FieldError::with_message("kategori_id", "Kategori wajib dipilih"));
    }
    if deskripsi.is_none() {
        errs.push(crate::errors::FieldError::with_message("deskripsi", "Deskripsi wajib diisi"));
    }
    if lokasi.is_none() || lokasi.as_deref().unwrap_or("").trim().is_empty() {
        errs.push(crate::errors::FieldError::with_message("lokasi", "Lokasi wajib diisi"));
    }
    if !errs.is_empty() {
        return Err(AppError::Validation(errs));
    }

    let input = crate::models::report::CreateReportInput {
        kategori_id: kategori_id.unwrap(),
        deskripsi:   deskripsi.unwrap(),
        lokasi:      lokasi.unwrap(),
        latitude,
        longitude,
        wilayah,
        foto_data,
    };

    let resp = report_svc::create_report(
        &state.pool,
        claims.sub,
        input,
        &state.config.upload_dir,
        state.config.max_upload_bytes,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(resp)))
}

/// GET /api/v1/reports/mine
pub async fn list_mine(
    State(state): State<AppState>,
    claims: Claims,
    Query(q): Query<ReportQuery>,
) -> Result<Json<PaginatedResponse<ReportSummary>>, AppError> {
    let resp = report_svc::get_mine(&state.pool, claims.sub, &q).await?;
    Ok(Json(resp))
}

/// GET /api/v1/reports/{id}
pub async fn get_detail(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<ReportDetail>, AppError> {
    let is_admin = claims.role == "admin";
    let detail = report_svc::get_detail_for_user(&state.pool, id, claims.sub, is_admin).await?;
    Ok(Json(detail))
}

/// GET /api/v1/reports/{id}/foto
pub async fn get_foto(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Response<Body>, AppError> {
    let is_admin = claims.role == "admin";
    let detail = report_svc::get_detail_for_user(&state.pool, id, claims.sub, is_admin).await?;

    let filename = detail.foto.ok_or(AppError::NotFound)?;
    let path = std::path::Path::new(&state.config.upload_dir).join(&filename);

    let data = tokio::fs::read(&path)
        .await
        .map_err(|_| AppError::NotFound)?;

    // Deteksi mime type dari magic bytes
    let mime = infer::get(&data)
        .map(|k| k.mime_type())
        .unwrap_or("application/octet-stream");

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime)
        .header(
            header::CONTENT_DISPOSITION,
            format!("inline; filename=\"{}\"", filename),
        )
        .body(Body::from(data))
        .map_err(|e| AppError::Internal(anyhow::anyhow!("{e}")))?;

    Ok(response)
}

