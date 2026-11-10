// tests/common/mod.rs – Helper untuk integration tests

use once_cell::sync::Lazy;

/// URL base server yang sedang diuji (dari env TEST_SERVER_URL atau spawn baru)
pub static SERVER_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("TEST_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string())
});

/// Helper: dapatkan token JWT untuk user yang sudah ada
pub async fn get_auth_token(client: &reqwest::Client, email: &str, password: &str) -> String {
    let resp = client
        .post(format!("{}/api/v1/auth/login", *SERVER_URL))
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .expect("Login request failed");

    assert_eq!(resp.status().as_u16(), 200, "Login harus 200");

    let body: serde_json::Value = resp.json().await.expect("Login response JSON");
    // Response: { "token": "...", "user": {...} }
    body["token"]
        .as_str()
        .expect("Token harus ada")
        .to_string()
}

/// Helper: buat request dengan Authorization header
pub fn auth_header(token: &str) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", token).parse().unwrap(),
    );
    headers
}
