// src/models/category.rs

use serde::Serialize;

/// Baris di tabel `categories`
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Category {
    pub id:            i16,
    pub nama_kategori: String,
}

