// src/models/user.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;
use validator::Validate;

/// Role pengguna – harus cocok dengan ENUM di PostgreSQL
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Masyarakat,
    Admin,
}

/// Baris di tabel `users`
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id:            Uuid,
    pub nama:          String,
    pub email:         String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role:          UserRole,
    pub created_at:    DateTime<Utc>,
}

/// Respons user (tanpa password_hash)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id:         Uuid,
    pub nama:       String,
    pub email:      String,
    pub role:       UserRole,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            nama: u.nama,
            email: u.email,
            role: u.role,
            created_at: u.created_at,
        }
    }
}

/// DTO: request registrasi
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 100, message = "Nama harus 2–100 karakter"))]
    pub nama: String,

    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}

/// DTO: request login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email:    String,
    pub password: String,
}

/// Respons login
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token:      String,
    pub expires_in: u64,
    pub user:       UserResponse,
}

/// Claims JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub:  Uuid,       // user_id
    pub role: String,     // "masyarakat" | "admin"
    pub exp:  usize,      // unix timestamp kadaluwarsa
    pub iat:  usize,      // unix timestamp diterbitkan
}

