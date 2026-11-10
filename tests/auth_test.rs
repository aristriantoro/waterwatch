// tests/auth_test.rs – Integration tests untuk endpoint autentikasi
// Jalankan setelah stack up: docker compose run --rm -e TEST_SERVER_URL=http://app:8080 app cargo test --test auth_test

mod common;
use common::*;

fn client() -> reqwest::Client { reqwest::Client::new() }

// ============================================================
// AC-01: Registrasi Warga
// ============================================================

#[tokio::test]
async fn test_register_success() {
    let c = client();
    let unique = &uuid::Uuid::new_v4().to_string()[..8];
    let resp = c
        .post(format!("{}/api/v1/auth/register", *SERVER_URL))
        .json(&serde_json::json!({
            "nama": "Test Warga",
            "email": format!("test_{}@example.com", unique),
            "password": "Password@1234"
        }))
        .send().await.expect("Register request failed");

    assert_eq!(resp.status().as_u16(), 201, "Register berhasil harus 201");
    // Handler register mengembalikan UserResponse { id, nama, email, role, created_at }
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["id"].is_string(), "id harus ada");
    assert_eq!(body["role"].as_str().unwrap(), "masyarakat");
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let c = client();
    let resp = c
        .post(format!("{}/api/v1/auth/register", *SERVER_URL))
        .json(&serde_json::json!({
            "nama": "Duplikat",
            "email": "warga1@example.com",
            "password": "Password@1234"
        }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 409, "Email duplikat harus 409");
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"].as_str().unwrap(), "CONFLICT");
}

#[tokio::test]
async fn test_register_validation_error() {
    let c = client();
    let resp = c
        .post(format!("{}/api/v1/auth/register", *SERVER_URL))
        .json(&serde_json::json!({
            "nama": "X",          // terlalu pendek
            "email": "bukan-email",
            "password": "123"     // terlalu pendek
        }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 422, "Validasi gagal harus 422");
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"].as_str().unwrap(), "VALIDATION_ERROR");
    assert!(body["error"]["details"].as_array().unwrap().len() > 0);
}

// ============================================================
// AC-02: Login
// ============================================================

#[tokio::test]
async fn test_login_success() {
    let c = client();
    let resp = c
        .post(format!("{}/api/v1/auth/login", *SERVER_URL))
        .json(&serde_json::json!({
            "email": "warga1@example.com",
            "password": "Warga@1234!"
        }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200, "Login valid harus 200");
    // Handler login: LoginResponse { token, expires_in, user }
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["token"].is_string(), "token harus ada");
    assert!(body["expires_in"].is_number(), "expires_in harus ada");
    assert_eq!(body["user"]["email"].as_str().unwrap(), "warga1@example.com");
}

#[tokio::test]
async fn test_login_wrong_password() {
    let c = client();
    let resp = c
        .post(format!("{}/api/v1/auth/login", *SERVER_URL))
        .json(&serde_json::json!({
            "email": "warga1@example.com",
            "password": "salah123"
        }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 401, "Password salah harus 401");
    let body: serde_json::Value = resp.json().await.unwrap();
    // Pesan harus generik (tidak menyebut email/password secara spesifik)
    assert_eq!(body["error"]["code"].as_str().unwrap(), "UNAUTHORIZED");
}

#[tokio::test]
async fn test_login_nonexistent_email() {
    let c = client();
    let resp = c
        .post(format!("{}/api/v1/auth/login", *SERVER_URL))
        .json(&serde_json::json!({
            "email": "tidakada99@example.com",
            "password": "Password@1234"
        }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 401, "Email tidak ada harus 401");
}

// ============================================================
// AC-03: GET /api/v1/auth/me
// ============================================================

#[tokio::test]
async fn test_me_with_valid_token() {
    let c = client();
    let token = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;

    let resp = c
        .get(format!("{}/api/v1/auth/me", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    // Handler me: UserResponse { id, nama, email, role, created_at }
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["email"].as_str().unwrap(), "warga1@example.com");
    assert_eq!(body["role"].as_str().unwrap(), "masyarakat");
}

#[tokio::test]
async fn test_me_without_token() {
    let c = client();
    let resp = c
        .get(format!("{}/api/v1/auth/me", *SERVER_URL))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 401);
}

#[tokio::test]
async fn test_me_with_invalid_token() {
    let c = client();
    let resp = c
        .get(format!("{}/api/v1/auth/me", *SERVER_URL))
        .header("Authorization", "Bearer token.palsu.xxx")
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 401);
}

// ============================================================
// AC-16: Kesehatan server
// ============================================================

#[tokio::test]
async fn test_health_endpoint() {
    let c = client();
    let resp = c
        .get(format!("{}/health", *SERVER_URL))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"].as_str().unwrap(), "ok");
}
