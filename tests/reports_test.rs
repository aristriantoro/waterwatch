// tests/reports_test.rs – Integration tests untuk endpoint laporan warga
// Jalankan setelah stack up: docker compose run --rm -e TEST_SERVER_URL=http://app:8080 app cargo test --test reports_test

mod common;
use common::*;

fn client() -> reqwest::Client { reqwest::Client::new() }

// ============================================================
// AC-05: Buat Laporan (F-03)
// POST /api/v1/reports → 201 CreateReportResponse { id, nomor, status, created_at }
// ============================================================

#[tokio::test]
async fn test_create_report_success() {
    let c = client();
    let token = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;

    let form = reqwest::multipart::Form::new()
        .text("kategori_id", "1")
        .text("deskripsi", "Air keruh sejak pagi, berbau tidak sedap dan tidak bisa digunakan untuk minum.")
        .text("lokasi", "Jl. Teuku Umar No. 10, Bandar Lampung")
        .text("wilayah", "Bandar Lampung");

    let resp = c
        .post(format!("{}/api/v1/reports", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 201, "Buat laporan harus 201");
    // CreateReportResponse { id, nomor, status, created_at }
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["id"].is_string(), "id harus ada");
    assert!(
        body["nomor"].as_str().unwrap_or("").starts_with("WW-"),
        "Nomor harus format WW-YYYYMMDD-NNNN"
    );
    assert_eq!(body["status"].as_str().unwrap(), "menunggu", "Status awal harus menunggu");
}

#[tokio::test]
async fn test_create_report_deskripsi_too_short() {
    let c = client();
    let token = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;

    let form = reqwest::multipart::Form::new()
        .text("kategori_id", "1")
        .text("deskripsi", "Pendek")   // < 10 karakter
        .text("lokasi", "Jl. Test No. 1");

    let resp = c
        .post(format!("{}/api/v1/reports", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 422, "Deskripsi pendek harus 422");
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["error"]["code"].as_str().unwrap(), "VALIDATION_ERROR");
}

#[tokio::test]
async fn test_create_report_no_auth() {
    let c = client();
    let form = reqwest::multipart::Form::new()
        .text("kategori_id", "1")
        .text("deskripsi", "Ini deskripsi yang cukup panjang untuk pengujian.")
        .text("lokasi", "Jl. Test No. 1");

    let resp = c
        .post(format!("{}/api/v1/reports", *SERVER_URL))
        .multipart(form)
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 401, "Tanpa token harus 401");
}

#[tokio::test]
async fn test_create_report_missing_required_fields() {
    let c = client();
    let token = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;

    // Hanya kirim kategori, tanpa deskripsi dan lokasi
    let form = reqwest::multipart::Form::new()
        .text("kategori_id", "1");

    let resp = c
        .post(format!("{}/api/v1/reports", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 422, "Field wajib kosong harus 422");
}

// ============================================================
// AC-06: Riwayat Laporan Warga (F-04)
// GET /api/v1/reports/mine → PaginatedResponse<ReportSummary> { data: [...], meta: {...} }
// ============================================================

#[tokio::test]
async fn test_list_my_reports() {
    let c = client();
    let token = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;

    let resp = c
        .get(format!("{}/api/v1/reports/mine", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body["data"].is_array(), "data harus array");
    assert!(body["meta"]["total"].is_number(), "meta.total harus ada");
    assert!(body["meta"]["page"].is_number(), "meta.page harus ada");
    assert!(body["meta"]["per_page"].is_number(), "meta.per_page harus ada");
}

#[tokio::test]
async fn test_list_my_reports_filter_status() {
    let c = client();
    let token = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;

    let resp = c
        .get(format!("{}/api/v1/reports/mine?status=menunggu&per_page=5", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    // Semua item di halaman ini harus berstatus menunggu
    if let Some(arr) = body["data"].as_array() {
        for r in arr {
            assert_eq!(r["status"].as_str().unwrap(), "menunggu",
                "Semua laporan harus menunggu jika difilter");
        }
    }
}

#[tokio::test]
async fn test_list_my_reports_no_auth() {
    let c = client();
    let resp = c
        .get(format!("{}/api/v1/reports/mine", *SERVER_URL))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 401);
}

// ============================================================
// AC-07: Detail Laporan – kepemilikan (F-04)
// GET /api/v1/reports/:id → ReportDetail { id, nomor, user_nama, kategori, ..., riwayat: [...] }
// ============================================================

#[tokio::test]
async fn test_get_report_detail_own() {
    let c = client();
    let token = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;

    // Buat laporan baru
    let form = reqwest::multipart::Form::new()
        .text("kategori_id", "2")
        .text("deskripsi", "Air tidak mengalir sama sekali sejak kemarin malam dan belum ada tindakan.")
        .text("lokasi", "Jl. Raden Intan No. 5, Bandar Lampung");

    let create_resp: serde_json::Value = c
        .post(format!("{}/api/v1/reports", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send().await.unwrap()
        .json().await.unwrap();

    let report_id = create_resp["id"].as_str().unwrap();

    let resp = c
        .get(format!("{}/api/v1/reports/{}", *SERVER_URL, report_id))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["id"].as_str().unwrap(), report_id);
    assert!(body["riwayat"].is_array(), "riwayat harus ada");
    // Riwayat pertama harus status menunggu
    assert_eq!(
        body["riwayat"][0]["status"].as_str().unwrap(),
        "menunggu"
    );
}

#[tokio::test]
async fn test_get_report_detail_other_user_returns_404() {
    // AC-07: warga lain akses laporan warga1 → 404 (jangan bocorkan keberadaan)
    let c = client();
    let token1 = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;
    let token2 = get_auth_token(&c, "warga2@example.com", "Warga@1234!").await;

    // warga1 buat laporan
    let form = reqwest::multipart::Form::new()
        .text("kategori_id", "1")
        .text("deskripsi", "Laporan rahasia milik warga1 yang tidak boleh dilihat orang lain.")
        .text("lokasi", "Lokasi Rahasia Warga1");

    let create_resp: serde_json::Value = c
        .post(format!("{}/api/v1/reports", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token1))
        .multipart(form)
        .send().await.unwrap()
        .json().await.unwrap();

    let report_id = create_resp["id"].as_str().unwrap();

    // warga2 coba akses → harus 404
    let resp = c
        .get(format!("{}/api/v1/reports/{}", *SERVER_URL, report_id))
        .header("Authorization", format!("Bearer {}", token2))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 404,
        "Warga lain akses laporan harus 404 (bukan 403, jangan bocorkan keberadaan)");
}

#[tokio::test]
async fn test_get_report_detail_nonexistent() {
    let c = client();
    let token = get_auth_token(&c, "warga1@example.com", "Warga@1234!").await;

    let resp = c
        .get(format!("{}/api/v1/reports/00000000-0000-0000-0000-000000000000", *SERVER_URL))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 404, "ID tidak ada harus 404");
}

// ============================================================
// AC-08: Kategori
// GET /api/v1/categories → { data: [ { id, nama_kategori } ] }
// ============================================================

#[tokio::test]
async fn test_get_categories() {
    let c = client();
    let resp = c
        .get(format!("{}/api/v1/categories", *SERVER_URL))
        .send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    let arr = body["data"].as_array().expect("data harus array");
    assert!(arr.len() >= 4, "Harus ada ≥4 kategori dari seed");
}
