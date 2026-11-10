// tests/admin_test.rs – Integration tests untuk endpoint admin
// Jalankan setelah stack up: docker compose run --rm -e TEST_SERVER_URL=http://app:8080 app cargo test --test admin_test

mod common;
use common::*;

fn client() -> reqwest::Client { reqwest::Client::new() }

// Kredensial dari src/bin/seed.rs
const ADMIN_EMAIL: &str = "admin@waterwatch.id";
const ADMIN_PASS:  &str = "Admin@1234!";

// Warga yang berbeda per test untuk isolasi
const W3: (&str, &str) = ("warga3@example.com",  "Warga@1234!");
const W4: (&str, &str) = ("warga4@example.com",  "Warga@1234!");
const W5: (&str, &str) = ("warga5@example.com",  "Warga@1234!");
const W6: (&str, &str) = ("warga6@example.com",  "Warga@1234!");
const W7: (&str, &str) = ("warga7@example.com",  "Warga@1234!");
const W8: (&str, &str) = ("warga8@example.com",  "Warga@1234!");

async fn new_report(c: &reqwest::Client, token: &str, kat: &str, desk: &str) -> String {
    let form = reqwest::multipart::Form::new()
        .text("kategori_id", kat.to_string())
        .text("deskripsi", desk.to_string())
        .text("lokasi", "Jl. Sudirman No. 1, Bandar Lampung");

    let resp: serde_json::Value = c
        .post(format!("{}/api/v1/reports", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send().await.unwrap()
        .json().await.unwrap();

    resp["id"].as_str().expect("id laporan harus ada").to_string()
}

// ============================================================
// AC-13: Dashboard Admin (F-06)
// GET /api/v1/admin/dashboard → DashboardResponse
// ============================================================

#[tokio::test]
async fn test_admin_dashboard_success() {
    let c = client();
    let token = get_auth_token(&c, ADMIN_EMAIL, ADMIN_PASS).await;

    let resp = c
        .get(format!("{}/api/v1/admin/dashboard", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    // DashboardResponse { total, per_status, hari_ini, per_kategori, laporan_terbaru }
    assert!(body["total"].is_number(), "total harus ada");
    assert!(body["per_status"]["menunggu"].is_number());
    assert!(body["per_status"]["diproses"].is_number());
    assert!(body["per_status"]["selesai"].is_number());
    assert!(body["per_kategori"].is_array(), "per_kategori harus array");
    assert!(body["laporan_terbaru"].is_array(), "laporan_terbaru harus array");
    // Seed membuat >=50 laporan
    assert!(body["total"].as_i64().unwrap() >= 50, "Harus ada ≥50 laporan dari seed");
}

#[tokio::test]
async fn test_admin_dashboard_forbidden_for_warga() {
    let c = client();
    let token = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;

    let resp = c
        .get(format!("{}/api/v1/admin/dashboard", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 403, "Warga akses endpoint admin harus 403");
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"].as_str().unwrap(), "FORBIDDEN");
}

#[tokio::test]
async fn test_admin_dashboard_no_auth() {
    let c = client();
    let resp = c
        .get(format!("{}/api/v1/admin/dashboard", *SERVER_URL))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 401);
}

// ============================================================
// AC-14: List Laporan Admin (F-07)
// GET /api/v1/admin/reports → PaginatedResponse<ReportSummaryAdmin>
// ============================================================

#[tokio::test]
async fn test_admin_list_reports() {
    let c = client();
    let token = get_auth_token(&c, ADMIN_EMAIL, ADMIN_PASS).await;

    let resp = c
        .get(format!("{}/api/v1/admin/reports", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["data"].is_array());
    // Setiap item harus punya user_nama (dari JOIN users)
    if let Some(arr) = body["data"].as_array() {
        if let Some(first) = arr.first() {
            assert!(first["user_nama"].is_string(), "user_nama harus ada di admin list");
        }
    }
    assert!(body["meta"]["total"].as_i64().unwrap() >= 50);
}

#[tokio::test]
async fn test_admin_list_reports_filter_status() {
    let c = client();
    let token = get_auth_token(&c, ADMIN_EMAIL, ADMIN_PASS).await;

    let resp = c
        .get(format!("{}/api/v1/admin/reports?status=menunggu&per_page=20", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    if let Some(arr) = body["data"].as_array() {
        for r in arr {
            assert_eq!(r["status"].as_str().unwrap(), "menunggu",
                "Filter status=menunggu harus hanya tampilkan menunggu");
        }
    }
}

#[tokio::test]
async fn test_admin_get_report_detail() {
    let c = client();
    let wtoken = get_auth_token(&c, W3.0, W3.1).await;
    let atoken = get_auth_token(&c, ADMIN_EMAIL, ADMIN_PASS).await;

    let rid = new_report(&c, &wtoken, "1",
        "Laporan untuk tes detail admin, air tidak mengalir sudah dua hari.").await;

    let resp = c
        .get(format!("{}/api/v1/admin/reports/{}", *SERVER_URL, rid))
        .header("Authorization", format!("Bearer {}", atoken))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["id"].as_str().unwrap(), rid);
    // Admin dapat melihat user_nama
    assert!(body["user_nama"].is_string(), "Admin harus dapat melihat user_nama");
    assert!(body["riwayat"].is_array());
}

// ============================================================
// AC-15: Update Status (F-08)
// PATCH /api/v1/admin/reports/:id/status → UpdateStatusResponse { id, status, updated_at }
// ============================================================

#[tokio::test]
async fn test_update_status_menunggu_to_diproses() {
    let c = client();
    let wtoken = get_auth_token(&c, W4.0, W4.1).await;
    let atoken = get_auth_token(&c, ADMIN_EMAIL, ADMIN_PASS).await;

    let rid = new_report(&c, &wtoken, "1",
        "Laporan untuk tes transisi valid menunggu ke diproses oleh admin.").await;

    let resp = c
        .patch(format!("{}/api/v1/admin/reports/{}/status", *SERVER_URL, rid))
        .header("Authorization", format!("Bearer {}", atoken))
        .json(&serde_json::json!({
            "status": "diproses",
            "keterangan": "Laporan sudah diterima dan sedang diproses oleh petugas lapangan."
        }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200, "Transisi menunggu→diproses harus 200");
    // UpdateStatusResponse { id, status, updated_at }
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"].as_str().unwrap(), "diproses");
    assert!(body["updated_at"].is_string());
}

#[tokio::test]
async fn test_update_status_invalid_transition_menunggu_to_selesai() {
    // menunggu → selesai tidak valid (harus lewat diproses)
    let c = client();
    let wtoken = get_auth_token(&c, W5.0, W5.1).await;
    let atoken = get_auth_token(&c, ADMIN_EMAIL, ADMIN_PASS).await;

    let rid = new_report(&c, &wtoken, "2",
        "Laporan untuk tes transisi tidak valid, langsung menunggu ke selesai.").await;

    let resp = c
        .patch(format!("{}/api/v1/admin/reports/{}/status", *SERVER_URL, rid))
        .header("Authorization", format!("Bearer {}", atoken))
        .json(&serde_json::json!({
            "status": "selesai",
            "keterangan": "Langsung selesai tanpa diproses."
        }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 422, "Transisi tidak valid harus 422");
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"].as_str().unwrap(), "INVALID_TRANSITION");
}

#[tokio::test]
async fn test_update_status_empty_keterangan() {
    let c = client();
    let wtoken = get_auth_token(&c, W6.0, W6.1).await;
    let atoken = get_auth_token(&c, ADMIN_EMAIL, ADMIN_PASS).await;

    let rid = new_report(&c, &wtoken, "3",
        "Laporan untuk tes keterangan kosong saat admin update status laporan.").await;

    let resp = c
        .patch(format!("{}/api/v1/admin/reports/{}/status", *SERVER_URL, rid))
        .header("Authorization", format!("Bearer {}", atoken))
        .json(&serde_json::json!({
            "status": "diproses",
            "keterangan": "   "   // hanya spasi → dianggap kosong
        }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 422, "Keterangan kosong/spasi harus 422");
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"].as_str().unwrap(), "VALIDATION_ERROR");
}

#[tokio::test]
async fn test_update_status_selesai_is_final() {
    // Status selesai adalah final → tidak bisa diubah lagi
    let c = client();
    let wtoken = get_auth_token(&c, W7.0, W7.1).await;
    let atoken = get_auth_token(&c, ADMIN_EMAIL, ADMIN_PASS).await;

    let rid = new_report(&c, &wtoken, "4",
        "Laporan untuk tes status selesai adalah final dan tidak bisa diubah.").await;

    // menunggu → diproses
    c.patch(format!("{}/api/v1/admin/reports/{}/status", *SERVER_URL, rid))
        .header("Authorization", format!("Bearer {}", atoken))
        .json(&serde_json::json!({ "status": "diproses", "keterangan": "Sedang diproses petugas." }))
        .send().await.unwrap();

    // diproses → selesai
    c.patch(format!("{}/api/v1/admin/reports/{}/status", *SERVER_URL, rid))
        .header("Authorization", format!("Bearer {}", atoken))
        .json(&serde_json::json!({ "status": "selesai", "keterangan": "Masalah telah diselesaikan." }))
        .send().await.unwrap();

    // Coba ubah lagi dari selesai → apapun → 422
    let resp = c
        .patch(format!("{}/api/v1/admin/reports/{}/status", *SERVER_URL, rid))
        .header("Authorization", format!("Bearer {}", atoken))
        .json(&serde_json::json!({ "status": "diproses", "keterangan": "Mencoba ubah dari selesai." }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 422, "Status selesai adalah final, ubah lagi harus 422");
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"].as_str().unwrap(), "INVALID_TRANSITION");
}

#[tokio::test]
async fn test_warga_cannot_update_status() {
    // Warga tidak bisa akses PATCH /admin/... → 403
    let c = client();
    let wtoken = get_auth_token(&c, W8.0, W8.1).await;

    let rid = new_report(&c, &wtoken, "1",
        "Laporan untuk tes warga tidak bisa update status laporan sendiri maupun orang lain.").await;

    let resp = c
        .patch(format!("{}/api/v1/admin/reports/{}/status", *SERVER_URL, rid))
        .header("Authorization", format!("Bearer {}", wtoken))
        .json(&serde_json::json!({ "status": "diproses", "keterangan": "Dicoba sendiri." }))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 403, "Warga update status harus 403");
}
