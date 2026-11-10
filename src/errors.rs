// src/errors.rs – Error handling terpadu: AppError → JSON response
// Format: { "error": { "code", "message", "details": [...] } }

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

/// Error tunggal untuk seluruh aplikasi
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Validasi gagal")]
    Validation(Vec<FieldError>),

    #[error("Tidak diotorisasi")]
    Unauthorized,

    #[error("Akses ditolak")]
    Forbidden,

    #[error("Data tidak ditemukan")]
    NotFound,

    #[error("Konflik data: {0}")]
    Conflict(String),

    #[error("File terlalu besar")]
    PayloadTooLarge,

    #[error("Tipe file tidak didukung")]
    UnsupportedMediaType,

    #[error("Transisi status tidak valid: {0}")]
    InvalidTransition(String),

    #[error("Terlalu banyak permintaan")]
    TooManyRequests,

    #[error("Kesalahan database: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Kesalahan internal: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize, Clone)]
pub struct FieldError {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl FieldError {
    pub fn new(field: impl Into<String>) -> Self {
        Self { field: field.into(), message: None }
    }
    pub fn with_message(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self { field: field.into(), message: Some(message.into()) }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, details): (StatusCode, &str, String, Vec<serde_json::Value>) =
            match &self {
                AppError::Validation(fields) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "VALIDATION_ERROR",
                    "Data tidak valid".into(),
                    fields
                        .iter()
                        .map(|f| json!({ "field": f.field, "message": f.message }))
                        .collect(),
                ),
                AppError::Unauthorized => (
                    StatusCode::UNAUTHORIZED,
                    "UNAUTHORIZED",
                    "Email atau password salah".into(),
                    vec![],
                ),
                AppError::Forbidden => (
                    StatusCode::FORBIDDEN,
                    "FORBIDDEN",
                    "Akses ditolak".into(),
                    vec![],
                ),
                AppError::NotFound => (
                    StatusCode::NOT_FOUND,
                    "NOT_FOUND",
                    "Data tidak ditemukan".into(),
                    vec![],
                ),
                AppError::Conflict(msg) => (
                    StatusCode::CONFLICT,
                    "CONFLICT",
                    msg.clone(),
                    vec![],
                ),
                AppError::PayloadTooLarge => (
                    StatusCode::PAYLOAD_TOO_LARGE,
                    "PAYLOAD_TOO_LARGE",
                    "Ukuran file melebihi 5 MB".into(),
                    vec![],
                ),
                AppError::UnsupportedMediaType => (
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    "UNSUPPORTED_MEDIA_TYPE",
                    "Hanya format JPG dan PNG yang didukung".into(),
                    vec![],
                ),
                AppError::InvalidTransition(msg) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "INVALID_TRANSITION",
                    msg.clone(),
                    vec![],
                ),
                AppError::TooManyRequests => (
                    StatusCode::TOO_MANY_REQUESTS,
                    "TOO_MANY_REQUESTS",
                    "Terlalu banyak percobaan. Coba lagi nanti.".into(),
                    vec![],
                ),
                AppError::Database(e) => {
                    tracing::error!("Database error: {e:#}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "INTERNAL_ERROR",
                        "Terjadi kesalahan pada server".into(),
                        vec![],
                    )
                }
                AppError::Internal(e) => {
                    tracing::error!("Internal error: {e:#}");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "INTERNAL_ERROR",
                        "Terjadi kesalahan pada server".into(),
                        vec![],
                    )
                }
            };

        (
            status,
            Json(json!({
                "error": {
                    "code": code,
                    "message": message,
                    "details": details
                }
            })),
        )
            .into_response()
    }
}

/// Helper: konversi sqlx::Error menjadi AppError yang tepat
pub fn db_error(e: sqlx::Error) -> AppError {
    match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        _ => AppError::Database(e),
    }
}

