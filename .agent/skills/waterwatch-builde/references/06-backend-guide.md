# Backend Guide – WaterWatch v1.0

> Stack: Rust + Axum 0.7 + SQLx + PostgreSQL 16

---

## 1. Struktur Proyek

```
waterwatch/
├── Cargo.toml
├── Cargo.lock
├── .env                      (dari .env.example, jangan di-commit)
├── .env.example
├── docker-compose.yml
├── nginx/
│   └── nginx.conf
├── migrations/
│   ├── 20261110000001_init.sql          (skema + ENUM + tabel)
│   └── 20261110000002_seed_categories.sql
├── uploads/                  (folder upload foto, dibuat runtime)
├── src/
│   ├── main.rs               (bootstrap server, router, state)
│   ├── config.rs             (env vars via envy/dotenvy)
│   ├── errors.rs             (AppError → JSON response)
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs           (User, UserRole, Claims JWT)
│   │   ├── report.rs         (Report, ReportStatus, CreateReportDto)
│   │   ├── category.rs
│   │   └── report_update.rs
│   ├── routes/
│   │   ├── mod.rs            (merge semua router)
│   │   ├── auth.rs
│   │   ├── reports.rs
│   │   ├── admin.rs
│   │   └── categories.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── reports.rs
│   │   ├── admin.rs
│   │   └── categories.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth.rs           (hash, verify, JWT)
│   │   ├── reports.rs        (logika bisnis, generate nomor)
│   │   └── admin.rs
│   ├── repositories/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── report.rs
│   │   └── category.rs
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs           (extract JWT, inject Claims)
│   │   └── role_guard.rs     (require_admin extractor)
│   └── bin/
│       └── seed.rs           (binary seed admin)
└── tests/
    ├── common/
    │   └── mod.rs            (helper test: spawn_app, create_test_user)
    ├── auth_test.rs
    ├── reports_test.rs
    └── admin_test.rs
```

---

## 2. Cargo.toml (dependensi utama)

```toml
[package]
name = "waterwatch"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "waterwatch"
path = "src/main.rs"

[[bin]]
name = "seed"
path = "src/bin/seed.rs"

[dependencies]
# Web framework
axum = { version = "0.7", features = ["multipart", "macros"] }
tokio = { version = "1", features = ["full"] }
tower = { version = "0.4" }
tower-http = { version = "0.5", features = [
    "cors", "trace", "compression-gzip", "fs", "set-headers"
] }
tower_governor = "0.3"

# Database
sqlx = { version = "0.7", features = [
    "runtime-tokio-rustls", "postgres", "uuid", "time", "chrono"
] }

# Auth
jsonwebtoken = "9"
argon2 = "0.5"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Validation
validator = { version = "0.18", features = ["derive"] }

# File type detection (magic bytes)
infer = "0.15"

# Utils
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
dotenvy = "0.15"
envy = "0.4"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1"
thiserror = "1"
bytes = "1"

[dev-dependencies]
reqwest = { version = "0.11", features = ["json", "multipart"] }
tokio = { version = "1", features = ["full"] }
```

---

## 3. Config (src/config.rs)

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_seconds: u64,          // default 3600
    pub upload_dir: String,               // default "./uploads"
    pub max_upload_bytes: u64,            // default 5242880 (5 MB)
    pub server_port: u16,                 // default 8080
    pub allowed_origins: String,          // default "http://localhost"
    pub rust_log: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenvy::dotenv().ok();
        envy::from_env()
    }
}
```

---

## 4. Error Handling (src/errors.rs)

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

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

    #[error("Konflik data")]
    Conflict(String),

    #[error("File terlalu besar")]
    PayloadTooLarge,

    #[error("Tipe file tidak didukung")]
    UnsupportedMediaType,

    #[error("Transisi status tidak valid")]
    InvalidTransition(String),

    #[error("Terlalu banyak permintaan")]
    TooManyRequests,

    #[error("Kesalahan internal: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, serde::Serialize)]
pub struct FieldError {
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, details) = match &self {
            AppError::Validation(fields) => (
                StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION_ERROR",
                "Data tidak valid",
                fields.iter().map(|f| json!({"field": f.field, "message": f.message})).collect(),
            ),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED",
                "Email atau password salah", vec![]),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "FORBIDDEN",
                "Akses ditolak", vec![]),
            AppError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND",
                "Data tidak ditemukan", vec![]),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT",
                msg.as_str(), vec![]),
            AppError::PayloadTooLarge => (StatusCode::PAYLOAD_TOO_LARGE, "PAYLOAD_TOO_LARGE",
                "Ukuran file melebihi 5 MB", vec![]),
            AppError::UnsupportedMediaType => (StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "UNSUPPORTED_MEDIA_TYPE", "Hanya format JPG dan PNG yang didukung", vec![]),
            AppError::InvalidTransition(msg) => (StatusCode::UNPROCESSABLE_ENTITY,
                "INVALID_TRANSITION", msg.as_str(), vec![]),
            AppError::TooManyRequests => (StatusCode::TOO_MANY_REQUESTS, "TOO_MANY_REQUESTS",
                "Terlalu banyak percobaan. Coba lagi nanti.", vec![]),
            AppError::Internal(e) => {
                tracing::error!("Internal error: {e:#}");
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR",
                 "Terjadi kesalahan pada server", vec![])
            }
        };

        (status, Json(json!({
            "error": {
                "code": code,
                "message": message,
                "details": details
            }
        }))).into_response()
    }
}
```

---

## 5. Logika Transisi Status

```rust
// src/services/reports.rs
use crate::models::report::ReportStatus;

pub fn validate_transition(current: &ReportStatus, next: &ReportStatus) -> bool {
    matches!(
        (current, next),
        (ReportStatus::Menunggu, ReportStatus::Diproses) |
        (ReportStatus::Diproses, ReportStatus::Selesai)
    )
}
```

---

## 6. Generate Nomor Laporan (Atomik, Dalam Transaksi)

```rust
// src/repositories/report.rs
pub async fn generate_report_number(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    today: chrono::NaiveDate,
) -> Result<String, AppError> {
    let row = sqlx::query!(
        r#"
        INSERT INTO report_number_seq (tanggal, counter)
        VALUES ($1, 1)
        ON CONFLICT (tanggal) DO UPDATE
            SET counter = report_number_seq.counter + 1
        RETURNING counter
        "#,
        today
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(format!("WW-{}-{:04}", today.format("%Y%m%d"), row.counter))
}
```

---

## 7. Upload Foto (Validasi Magic Bytes)

```rust
// src/handlers/reports.rs (excerpt)
use infer;

pub async fn validate_and_save_photo(
    data: &[u8],
    upload_dir: &str,
    max_bytes: u64,
) -> Result<String, AppError> {
    // Cek ukuran
    if data.len() as u64 > max_bytes {
        return Err(AppError::PayloadTooLarge);
    }

    // Cek tipe via magic bytes (bukan ekstensi)
    let kind = infer::get(data).ok_or(AppError::UnsupportedMediaType)?;
    let ext = match kind.mime_type() {
        "image/jpeg" => "jpg",
        "image/png"  => "png",
        _            => return Err(AppError::UnsupportedMediaType),
    };

    // Simpan dengan nama UUID
    let filename = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let path = std::path::Path::new(upload_dir).join(&filename);
    tokio::fs::write(&path, data).await.map_err(|e| AppError::Internal(e.into()))?;

    // Kembalikan path relatif (disimpan di DB)
    Ok(filename)
}
```

---

## 8. JWT Middleware

```rust
// src/middleware/auth.rs
use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::models::user::Claims;
use crate::errors::AppError;

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims where S: Send + Sync {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth = parts.headers.get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(AppError::Unauthorized)?;

        // Ambil jwt_secret dari state
        let app_state = crate::AppState::from_ref(state); // via FromRef
        let token_data = decode::<Claims>(
            auth,
            &DecodingKey::from_secret(app_state.config.jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        ).map_err(|_| AppError::Unauthorized)?;

        Ok(token_data.claims)
    }
}
```

---

## 9. Aturan Keamanan Wajib

1. **SQL**: Selalu gunakan `sqlx::query!` / `query_as!` / `.bind()`. **DILARANG** string concatenation SQL.
2. **Password**: Hanya hash Argon2id. `argon2::Argon2::default().hash_password(pwd, &salt)`.
3. **Login gagal**: Selalu return 401 dengan pesan sama ("Email atau password salah"), baik email tidak ada maupun password salah.
4. **Upload**: Validasi magic bytes, bukan ekstensi. Nama file UUID. Folder `/uploads` TIDAK dieksekusi.
5. **Warga vs laporan**: Jika `report.user_id != claims.sub`, return 404 (bukan 403) untuk tidak bocorkan keberadaan.
6. **Admin route**: Semua `/admin/*` wajib cek `claims.role == "admin"`. Jika bukan → 403.
7. **Tidak ada `unwrap()`** di jalur request. Gunakan `?` dan propagasi error ke `AppError`.
8. **CORS**: Batasi ke origin sendiri. Set header keamanan: `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy: strict-origin-when-cross-origin`, CSP dasar.

---

## 10. Rate Limiting Login

```rust
// src/routes/auth.rs (excerpt)
use tower_governor::{GovernorConfig, GovernorConfigBuilder, GovernorLayer};
use std::net::IpAddr;

// Max 5 request per 60 detik per IP untuk login
let login_governor = Arc::new(
    GovernorConfigBuilder::default()
        .per_second(60)
        .burst_size(5)
        .finish()
        .unwrap()
);

Router::new()
    .route("/login", post(handlers::auth::login))
    .layer(GovernorLayer { config: login_governor })
```

---

## 11. Seed Admin (src/bin/seed.rs)

```rust
// Binary terpisah – dijalankan sekali: `cargo run --bin seed`
// Membuat 1 admin default + data simulasi (Fase 4)

#[tokio::main]
async fn main() {
    let config = Config::from_env().expect("Config error");
    let pool = PgPool::connect(&config.database_url).await.expect("DB error");

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(b"admin1234", &salt)
        .unwrap()
        .to_string();

    sqlx::query!(
        "INSERT INTO users (nama, email, password_hash, role) VALUES ($1, $2, $3, 'admin')
         ON CONFLICT (email) DO NOTHING",
        "Admin WaterWatch", "admin@waterwatch.id", hash
    )
    .execute(&pool)
    .await
    .expect("Seed admin gagal");

    println!("Admin seeded: admin@waterwatch.id / admin1234");
}
```

---

## 12. Health Check

```rust
// src/routes/mod.rs
Router::new().route("/health", get(|| async {
    Json(serde_json::json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION") }))
}))
```

---

## 13. Tes Integrasi (pola)

```rust
// tests/auth_test.rs
#[tokio::test]
async fn register_happy_path() {
    let app = spawn_app().await;  // spawn server test + DB sementara
    let client = reqwest::Client::new();

    let res = client.post(format!("{}/api/v1/auth/register", app.base_url))
        .json(&serde_json::json!({
            "nama": "Test User",
            "email": "test@example.com",
            "password": "password123"
        }))
        .send().await.unwrap();

    assert_eq!(res.status(), 201);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["role"], "masyarakat");
}

#[tokio::test]
async fn register_duplicate_email_returns_409() {
    // ... seed user, kirim register dengan email sama, assert 409
}
```

