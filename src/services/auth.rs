// src/services/auth.rs – Logika bisnis autentikasi

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::user::{Claims, LoginRequest, LoginResponse, RegisterRequest, UserResponse, UserRole},
    repositories::user as user_repo,
};

pub async fn register(pool: &PgPool, req: RegisterRequest) -> Result<UserResponse, AppError> {
    // Validasi input
    use validator::Validate;
    req.validate().map_err(|e| {
        let fields = e
            .field_errors()
            .into_iter()
            .flat_map(|(field, errs)| {
                errs.iter().map(move |err| crate::errors::FieldError::with_message(
                    field,
                    err.message.as_deref().unwrap_or("Tidak valid").to_string(),
                ))
            })
            .collect();
        AppError::Validation(fields)
    })?;

    // Hash password dengan Argon2id
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Hash error: {e}")))?
        .to_string();

    let user = user_repo::insert_user(
        pool,
        &req.nama,
        &req.email.to_lowercase(),
        &password_hash,
        UserRole::Masyarakat,
    )
    .await?;

    Ok(user.into())
}

pub async fn login(
    pool: &PgPool,
    req: LoginRequest,
    jwt_secret: &str,
    jwt_expiry_seconds: u64,
) -> Result<LoginResponse, AppError> {
    // Selalu return 401 generik untuk email tidak ada / password salah
    let user = user_repo::find_by_email(pool, &req.email.to_lowercase())
        .await?
        .ok_or(AppError::Unauthorized)?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Unauthorized)?;
    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;

    let role_str = match user.role {
        UserRole::Admin => "admin",
        UserRole::Masyarakat => "masyarakat",
    };

    let now = Utc::now();
    let exp = (now + chrono::Duration::seconds(jwt_expiry_seconds as i64)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        role: role_str.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encode error: {e}")))?;

    Ok(LoginResponse {
        token,
        expires_in: jwt_expiry_seconds,
        user: user.into(),
    })
}

pub async fn get_me(pool: &PgPool, user_id: Uuid) -> Result<UserResponse, AppError> {
    let user = user_repo::find_by_id(pool, user_id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(user.into())
}

