// src/config.rs – Konfigurasi aplikasi dari environment variables

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// URL koneksi database, contoh: postgres://user:pass@host:5432/db
    pub database_url: String,

    /// Secret untuk signing JWT
    pub jwt_secret: String,

    /// Masa berlaku JWT dalam detik (default 3600 = 1 jam)
    #[serde(default = "default_jwt_expiry")]
    pub jwt_expiry_seconds: u64,

    /// Folder penyimpanan foto (absolute path di container)
    #[serde(default = "default_upload_dir")]
    pub upload_dir: String,

    /// Ukuran maksimum upload foto dalam byte (default 5 MB)
    #[serde(default = "default_max_upload")]
    pub max_upload_bytes: u64,

    /// Port server
    #[serde(default = "default_port")]
    pub server_port: u16,

    /// Origin yang diizinkan untuk CORS (pisah koma)
    #[serde(default = "default_origins")]
    pub allowed_origins: String,

    /// Burst size rate limiter login (default 5, naikkan untuk test)
    #[serde(default = "default_rate_limit_burst")]
    pub rate_limit_burst: u32,

    /// Level logging (RUST_LOG)
    pub rust_log: Option<String>,
}

fn default_jwt_expiry() -> u64 { 3600 }
fn default_upload_dir() -> String { "./uploads".into() }
fn default_max_upload() -> u64 { 5 * 1024 * 1024 } // 5 MB
fn default_port() -> u16 { 8080 }
fn default_origins() -> String { "http://localhost".into() }
fn default_rate_limit_burst() -> u32 { 5 }

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        let mut cfg = envy::from_env::<Config>()
            .map_err(|e| anyhow::anyhow!("Konfigurasi tidak valid: {e}"))?;

        // Dukungan PORT bawaan cloud hosting (Render, Railway, Fly.io, Heroku)
        if let Ok(port_env) = std::env::var("PORT") {
            if let Ok(p) = port_env.parse::<u16>() {
                cfg.server_port = p;
            }
        }

        Ok(cfg)
    }
}

