// src/repositories/user.rs – Query DB untuk tabel users

use sqlx::PgPool;
use uuid::Uuid;
use crate::{errors::AppError, models::user::{User, UserRole}};

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, nama, email, password_hash,
                  role AS "role: UserRole", created_at
           FROM users WHERE email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(user)
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, nama, email, password_hash,
                  role AS "role: UserRole", created_at
           FROM users WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(user)
}

pub async fn insert_user(
    pool: &PgPool,
    nama: &str,
    email: &str,
    password_hash: &str,
    role: UserRole,
) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (nama, email, password_hash, role)
           VALUES ($1, $2, $3, $4)
           RETURNING id, nama, email, password_hash,
                     role AS "role: UserRole", created_at"#,
        nama,
        email,
        password_hash,
        role as UserRole,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.constraint() == Some("users_email_key") => {
            AppError::Conflict("Email sudah terdaftar".into())
        }
        _ => AppError::Database(e),
    })?;
    Ok(user)
}

