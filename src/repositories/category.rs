// src/repositories/category.rs

use sqlx::PgPool;
use crate::{errors::AppError, models::category::Category};

pub async fn find_all(pool: &PgPool) -> Result<Vec<Category>, AppError> {
    let cats = sqlx::query_as!(
        Category,
        "SELECT id, nama_kategori FROM categories ORDER BY id"
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(cats)
}

pub async fn find_by_id(pool: &PgPool, id: i16) -> Result<Option<Category>, AppError> {
    let cat = sqlx::query_as!(
        Category,
        "SELECT id, nama_kategori FROM categories WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(cat)
}

