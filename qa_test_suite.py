# qa_test_suite.py
import urllib.request
import urllib.parse
import urllib.error
import json
import time
import uuid
import mimetypes

BASE_URL = "http://localhost"

results = []

def record(test_id, category, name, expected, actual, status, details=""):
    results.append({
        "id": test_id,
        "category": category,
        "name": name,
        "expected": expected,
        "actual": actual,
        "status": status,
        "details": details
    })
    print(f"[{status}] {test_id}: {name}")

def http_req(path, method="GET", data=None, headers=None):
    url = f"{BASE_URL}{path}"
    req_headers = {"User-Agent": "WaterWatch-QA/1.0"}
    if headers:
        req_headers.update(headers)
    
    encoded_data = None
    if data is not None:
        if isinstance(data, dict):
            encoded_data = json.dumps(data).encode("utf-8")
            req_headers["Content-Type"] = "application/json"
        elif isinstance(data, (bytes, bytearray)):
            encoded_data = data
        else:
            encoded_data = str(data).encode("utf-8")
            
    req = urllib.request.Request(url, data=encoded_data, headers=req_headers, method=method)
    
    start_time = time.time()
    try:
        with urllib.request.urlopen(req, timeout=10) as res:
            elapsed_ms = (time.time() - start_time) * 1000
            body_bytes = res.read()
            try:
                body_json = json.loads(body_bytes.decode("utf-8"))
            except Exception:
                body_json = body_bytes.decode("utf-8", errors="replace")
            return {
                "status_code": res.status,
                "headers": dict(res.headers),
                "body": body_json,
                "elapsed_ms": elapsed_ms
            }
    except urllib.error.HTTPError as e:
        elapsed_ms = (time.time() - start_time) * 1000
        body_bytes = e.read()
        try:
            body_json = json.loads(body_bytes.decode("utf-8"))
        except Exception:
            body_json = body_bytes.decode("utf-8", errors="replace")
        return {
            "status_code": e.code,
            "headers": dict(e.headers),
            "body": body_json,
            "elapsed_ms": elapsed_ms
        }
    except Exception as e:
        elapsed_ms = (time.time() - start_time) * 1000
        return {
            "status_code": 0,
            "headers": {},
            "body": str(e),
            "elapsed_ms": elapsed_ms
        }

def build_multipart(fields, files=None):
    boundary = "----WebKitFormBoundary" + uuid.uuid4().hex
    body = bytearray()
    
    for k, v in fields.items():
        body.extend(f"--{boundary}\r\n".encode("utf-8"))
        body.extend(f'Content-Disposition: form-data; name="{k}"\r\n\r\n'.encode("utf-8"))
        body.extend(f"{v}\r\n".encode("utf-8"))
        
    if files:
        for field_name, (filename, content, content_type) in files.items():
            body.extend(f"--{boundary}\r\n".encode("utf-8"))
            body.extend(f'Content-Disposition: form-data; name="{field_name}"; filename="{filename}"\r\n'.encode("utf-8"))
            body.extend(f"Content-Type: {content_type}\r\n\r\n".encode("utf-8"))
            body.extend(content)
            body.extend(b"\r\n")
            
    body.extend(f"--{boundary}--\r\n".encode("utf-8"))
    return bytes(body), f"multipart/form-data; boundary={boundary}"

def run_tests():
    print("=== MEMULAI QA TESTING SUITE WATERWATCH ===")
    
    # ----------------------------------------------------
    # 1. API TESTING: HEALTH & SECURITY HEADERS
    # ----------------------------------------------------
    r = http_req("/health")
    headers = r["headers"]
    has_nosniff = headers.get("x-content-type-options") == "nosniff" or headers.get("X-Content-Type-Options") == "nosniff"
    has_xframe = "DENY" in (headers.get("x-frame-options") or headers.get("X-Frame-Options") or "")
    
    record("API-01", "API Testing", "Pemeriksaan Endpoint Health Check", 
           "HTTP 200, status: ok", f"HTTP {r['status_code']}, body: {r['body']}",
           "PASS" if r["status_code"] == 200 and r["body"].get("status") == "ok" else "FAIL")
           
    record("SEC-01", "Security Testing", "Pemeriksaan Header Keamanan HTTP", 
           "x-content-type-options: nosniff, x-frame-options: DENY",
           f"x-content-type-options: {headers.get('x-content-type-options')}, x-frame-options: {headers.get('x-frame-options')}",
           "PASS" if has_nosniff and has_xframe else "FAIL")

    # ----------------------------------------------------
    # 2. FUNCTIONAL & API: AUTHENTICATION (F-01)
    # ----------------------------------------------------
    # Register valid
    test_email = f"qa_warga_{uuid.uuid4().hex[:6]}@example.com"
    r = http_req("/api/v1/auth/register", method="POST", data={
        "nama": "QA Tester Warga",
        "email": test_email,
        "password": "Password@1234"
    })
    record("AUTH-01", "Functional Testing", "Registrasi Warga Baru Valid",
           "HTTP 201, UserResponse dengan role masyarakat",
           f"HTTP {r['status_code']}, role: {r['body'].get('role') if isinstance(r['body'], dict) else 'none'}",
           "PASS" if r["status_code"] == 201 and r["body"].get("role") == "masyarakat" else "FAIL")

    # Register duplicate
    r = http_req("/api/v1/auth/register", method="POST", data={
        "nama": "QA Tester Duplikat",
        "email": test_email,
        "password": "Password@1234"
    })
    record("AUTH-02", "Functional Testing", "Registrasi Email Duplikat",
           "HTTP 409 CONFLICT",
           f"HTTP {r['status_code']}, code: {r['body'].get('error', {}).get('code') if isinstance(r['body'], dict) else 'none'}",
           "PASS" if r["status_code"] == 409 else "FAIL")

    # Register validation: password too short (< 8)
    r = http_req("/api/v1/auth/register", method="POST", data={
        "nama": "QA Invalid",
        "email": "invalid_qa@example.com",
        "password": "123"
    })
    record("AUTH-03", "Black Box Testing", "Registrasi Password Terlalu Pendek (< 8 kar)",
           "HTTP 422 VALIDATION_ERROR",
           f"HTTP {r['status_code']}",
           "PASS" if r["status_code"] == 422 else "FAIL")

    # Register validation: format email salah
    r = http_req("/api/v1/auth/register", method="POST", data={
        "nama": "QA Invalid",
        "email": "bukan-email-valid",
        "password": "Password@1234"
    })
    record("AUTH-04", "Black Box Testing", "Registrasi Format Email Tidak Valid",
           "HTTP 422 VALIDATION_ERROR",
           f"HTTP {r['status_code']}",
           "PASS" if r["status_code"] == 422 else "FAIL")

    # Login Sukses Warga
    r = http_req("/api/v1/auth/login", method="POST", data={
        "email": test_email,
        "password": "Password@1234"
    })
    warga_token = r["body"].get("token") if isinstance(r["body"], dict) else None
    record("AUTH-05", "Functional Testing", "Login Warga Kredensial Valid",
           "HTTP 200, JWT token dikembalikan",
           f"HTTP {r['status_code']}, token_ada: {bool(warga_token)}",
           "PASS" if r["status_code"] == 200 and warga_token else "FAIL")

    # Login Password Salah
    r = http_req("/api/v1/auth/login", method="POST", data={
        "email": test_email,
        "password": "PasswordSalah123"
    })
    err_msg = r["body"].get("error", {}).get("message") if isinstance(r["body"], dict) else ""
    record("AUTH-06", "Security Testing", "Login Password Salah (Pesan Generik)",
           "HTTP 401 UNAUTHORIZED, pesan generik",
           f"HTTP {r['status_code']}, msg: '{err_msg}'",
           "PASS" if r["status_code"] == 401 and "Email atau password salah" in err_msg else "FAIL")

    # Login Email Tidak Terdaftar
    r = http_req("/api/v1/auth/login", method="POST", data={
        "email": "tidak_ada_di_database_999@example.com",
        "password": "Password@1234"
    })
    record("AUTH-07", "Security Testing", "Login Email Tidak Terdaftar (Pesan Sama)",
           "HTTP 401 UNAUTHORIZED, pesan generik sama",
           f"HTTP {r['status_code']}",
           "PASS" if r["status_code"] == 401 else "FAIL")

    # Login Sukses Admin
    r = http_req("/api/v1/auth/login", method="POST", data={
        "email": "admin@waterwatch.id",
        "password": "Admin@1234!"
    })
    admin_token = r["body"].get("token") if isinstance(r["body"], dict) else None
    admin_role = r["body"].get("user", {}).get("role") if isinstance(r["body"], dict) else None
    record("AUTH-08", "Functional Testing", "Login Admin Utama Kredensial Valid",
           "HTTP 200, role admin",
           f"HTTP {r['status_code']}, role: {admin_role}",
           "PASS" if r["status_code"] == 200 and admin_role == "admin" else "FAIL")

    # GET /auth/me dengan token warga
    r = http_req("/api/v1/auth/me", headers={"Authorization": f"Bearer {warga_token}"})
    record("AUTH-09", "API Testing", "Verifikasi Profile Diri (/auth/me)",
           "HTTP 200, data profil warga",
           f"HTTP {r['status_code']}, email: {r['body'].get('email') if isinstance(r['body'], dict) else ''}",
           "PASS" if r["status_code"] == 200 and r["body"].get("email") == test_email else "FAIL")

    # GET /auth/me tanpa token
    r = http_req("/api/v1/auth/me")
    record("AUTH-10", "Security Testing", "Akses Protected Endpoint Tanpa Token",
           "HTTP 401 UNAUTHORIZED",
           f"HTTP {r['status_code']}",
           "PASS" if r["status_code"] == 401 else "FAIL")

    # ----------------------------------------------------
    # 3. KATEGORI & PELAPORAN (F-02, F-03, F-04)
    # ----------------------------------------------------
    # Ambil Kategori
    r = http_req("/api/v1/categories")
    cat_list = r["body"].get("data", []) if isinstance(r["body"], dict) else []
    record("CAT-01", "Functional Testing", "Pengambilan Daftar Kategori Gangguan Air",
           "HTTP 200, minimal 4 kategori standar",
           f"HTTP {r['status_code']}, jumlah: {len(cat_list)}",
           "PASS" if r["status_code"] == 200 and len(cat_list) >= 4 else "FAIL")

    # Buat Laporan Valid (Tanpa Foto)
    body_data, ct = build_multipart({
        "kategori_id": "1",
        "deskripsi": "Pipa distribusi utama bocor menyebabkan semburan air di badan jalan raya.",
        "lokasi": "Jl. Teuku Umar No. 12, Kedaton",
        "wilayah": "Kedaton"
    })
    r = http_req("/api/v1/reports", method="POST", data=body_data, headers={
        "Authorization": f"Bearer {warga_token}",
        "Content-Type": ct
    })
    created_id = r["body"].get("id") if isinstance(r["body"], dict) else None
    created_nomor = r["body"].get("nomor") if isinstance(r["body"], dict) else ""
    created_status = r["body"].get("status") if isinstance(r["body"], dict) else ""
    record("REP-01", "Functional Testing", "Pembuatan Laporan Baru Valid",
           "HTTP 201, nomor WW-YYYYMMDD-NNNN, status menunggu",
           f"HTTP {r['status_code']}, nomor: '{created_nomor}', status: '{created_status}'",
           "PASS" if r["status_code"] == 201 and created_nomor.startswith("WW-") and created_status == "menunggu" else "FAIL")

    # Buat Laporan dengan Foto Valid (1x1 PNG dummy magic bytes)
    png_bytes = bytes([
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, # PNG magic
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4,
        0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41,
        0x54, 0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00,
        0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
        0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE,
        0x42, 0x60, 0x82
    ])
    body_data, ct = build_multipart({
        "kategori_id": "2",
        "deskripsi": "Air PDAM keruh dan berbau karat sejak pagi hari tidak bisa digunakan.",
        "lokasi": "Jl. ZA Pagar Alam No. 45, Rajabasa",
        "latitude": "-5.3789",
        "longitude": "105.2534"
    }, files={"foto": ("bukti.png", png_bytes, "image/png")})
    r = http_req("/api/v1/reports", method="POST", data=body_data, headers={
        "Authorization": f"Bearer {warga_token}",
        "Content-Type": ct
    })
    rep_with_photo_id = r["body"].get("id") if isinstance(r["body"], dict) else None
    record("REP-02", "Functional Testing", "Pembuatan Laporan dengan Lampiran Foto PNG & Koordinat",
           "HTTP 201 Created",
           f"HTTP {r['status_code']}, ID: {rep_with_photo_id}",
           "PASS" if r["status_code"] == 201 and rep_with_photo_id else "FAIL")

    # Unduh Foto Laporan
    r = http_req(f"/api/v1/reports/{rep_with_photo_id}/foto", headers={"Authorization": f"Bearer {warga_token}"})
    ct_header = r["headers"].get("Content-Type") or r["headers"].get("content-type") or ""
    record("REP-03", "API Testing", "Akses Berkas Foto Laporan (/reports/:id/foto)",
           "HTTP 200, Content-Type image/png",
           f"HTTP {r['status_code']}, mime: {ct_header}",
           "PASS" if r["status_code"] == 200 and "image/png" in ct_header else "FAIL")

    # Upload Foto Palsu (Magic Bytes bukan gambar, misal text plain)
    fake_img = b"INI BUKAN GAMBAR TAPI TEKS BIASA"
    body_data, ct = build_multipart({
        "kategori_id": "1",
        "deskripsi": "Mencoba mengunggah berkas teks dengan ekstensi foto palsu.",
        "lokasi": "Lokasi Test Malicious"
    }, files={"foto": ("malicious.png", fake_img, "image/png")})
    r = http_req("/api/v1/reports", method="POST", data=body_data, headers={
        "Authorization": f"Bearer {warga_token}",
        "Content-Type": ct
    })
    record("SEC-02", "Security Testing", "Validasi Magic Bytes Foto Palsu (Anti-Spoofing)",
           "HTTP 415 UNSUPPORTED_MEDIA_TYPE",
           f"HTTP {r['status_code']}, code: {r['body'].get('error', {}).get('code') if isinstance(r['body'], dict) else ''}",
           "PASS" if r["status_code"] == 415 else "FAIL")

    # Boundary Testing: Deskripsi Terlalu Pendek (< 10 karakter)
    body_data, ct = build_multipart({
        "kategori_id": "1",
        "deskripsi": "Air mati", # 8 karakter
        "lokasi": "Jl. Mawar No. 1"
    })
    r = http_req("/api/v1/reports", method="POST", data=body_data, headers={
        "Authorization": f"Bearer {warga_token}",
        "Content-Type": ct
    })
    record("BND-01", "Black Box Testing", "Validasi Batas Panjang Deskripsi (< 10 kar)",
           "HTTP 422 VALIDATION_ERROR",
           f"HTTP {r['status_code']}",
           "PASS" if r["status_code"] == 422 else "FAIL")

    # Boundary Testing: Koordinat di Luar Jangkauan (-95 lat)
    body_data, ct = build_multipart({
        "kategori_id": "1",
        "deskripsi": "Deskripsi laporan yang panjangnya lebih dari sepuluh karakter.",
        "lokasi": "Jl. Antariksa",
        "latitude": "-95.0",
        "longitude": "105.0"
    })
    r = http_req("/api/v1/reports", method="POST", data=body_data, headers={
        "Authorization": f"Bearer {warga_token}",
        "Content-Type": ct
    })
    record("BND-02", "Black Box Testing", "Validasi Batas Koordinat Latitude (-90..90)",
           "HTTP 422 VALIDATION_ERROR",
           f"HTTP {r['status_code']}",
           "PASS" if r["status_code"] == 422 else "FAIL")

    # ----------------------------------------------------
    # 4. RIWAYAT & ACCESS CONTROL (F-05)
    # ----------------------------------------------------
    # Riwayat Laporan Milik Sendiri
    r = http_req("/api/v1/reports/mine", headers={"Authorization": f"Bearer {warga_token}"})
    mine_data = r["body"].get("data", []) if isinstance(r["body"], dict) else []
    record("ACC-01", "Functional Testing", "Daftar Riwayat Laporan Milik Sendiri",
           "HTTP 200, berisi laporan yang baru dibuat",
           f"HTTP {r['status_code']}, total_items: {len(mine_data)}",
           "PASS" if r["status_code"] == 200 and len(mine_data) >= 2 else "FAIL")

    # Detail Laporan Milik Sendiri
    r = http_req(f"/api/v1/reports/{created_id}", headers={"Authorization": f"Bearer {warga_token}"})
    has_riwayat = len(r["body"].get("riwayat", [])) >= 1 if isinstance(r["body"], dict) else False
    record("ACC-02", "Functional Testing", "Detail Laporan Sendiri & Riwayat Perubahan",
           "HTTP 200, memiliki array riwayat status",
           f"HTTP {r['status_code']}, riwayat_len: {len(r['body'].get('riwayat', [])) if isinstance(r['body'], dict) else 0}",
           "PASS" if r["status_code"] == 200 and has_riwayat else "FAIL")

    # ACCESS CONTROL IDOR: Warga lain mencoba akses laporan created_id
    r_warga2 = http_req("/api/v1/auth/login", method="POST", data={
        "email": "warga2@example.com",
        "password": "Warga@1234!"
    })
    warga2_token = r_warga2["body"].get("token")
    r = http_req(f"/api/v1/reports/{created_id}", headers={"Authorization": f"Bearer {warga2_token}"})
    record("SEC-03", "Security Testing", "Perlindungan Akses IDOR (Warga Lain -> Laporan Orang Lain)",
           "HTTP 404 NOT FOUND (Bukan 403 untuk mencegah kebocoran ID)",
           f"HTTP {r['status_code']}",
           "PASS" if r["status_code"] == 404 else "FAIL")

    # ----------------------------------------------------
    # 5. DASHBOARD & FITUR ADMIN (F-06, F-07, F-08)
    # ----------------------------------------------------
    # Warga coba akses dashboard admin
    r = http_req("/api/v1/admin/dashboard", headers={"Authorization": f"Bearer {warga_token}"})
    record("SEC-04", "Security Testing", "Warga Biasa Akses Endpoint Admin",
           "HTTP 403 FORBIDDEN",
           f"HTTP {r['status_code']}",
           "PASS" if r["status_code"] == 403 else "FAIL")

    # Admin Dashboard Sukses
    r = http_req("/api/v1/admin/dashboard", headers={"Authorization": f"Bearer {admin_token}"})
    db_total = r["body"].get("total", 0) if isinstance(r["body"], dict) else 0
    has_status_counts = "menunggu" in r["body"].get("per_status", {}) if isinstance(r["body"], dict) else False
    record("ADM-01", "Functional Testing", "Pengambilan Data Dashboard Admin",
           "HTTP 200, total laporan > 0, rincian per status",
           f"HTTP {r['status_code']}, total: {db_total}, has_status_counts: {has_status_counts}",
           "PASS" if r["status_code"] == 200 and db_total > 0 and has_status_counts else "FAIL")

    # Admin List Reports dengan Filter
    r = http_req("/api/v1/admin/reports?status=menunggu&per_page=5", headers={"Authorization": f"Bearer {admin_token}"})
    reports_data = r["body"].get("data", []) if isinstance(r["body"], dict) else []
    all_menunggu = all(item.get("status") == "menunggu" for item in reports_data) if reports_data else True
    has_user_nama = all("user_nama" in item for item in reports_data) if reports_data else False
    record("ADM-02", "Functional Testing", "Filter Status Laporan Admin & JOIN User Nama",
           "HTTP 200, semua status menunggu, memuat user_nama",
           f"HTTP {r['status_code']}, all_menunggu: {all_menunggu}, has_user_nama: {has_user_nama}",
           "PASS" if r["status_code"] == 200 and all_menunggu and has_user_nama else "FAIL")

    # Admin Detail Laporan Warga
    r = http_req(f"/api/v1/admin/reports/{created_id}", headers={"Authorization": f"Bearer {admin_token}"})
    record("ADM-03", "Functional Testing", "Admin Melihat Detail Laporan Lengkap Warga",
           "HTTP 200, nama warga pembuat laporan tampil",
           f"HTTP {r['status_code']}, user_nama: {r['body'].get('user_nama') if isinstance(r['body'], dict) else ''}",
           "PASS" if r["status_code"] == 200 and r["body"].get("user_nama") == "QA Tester Warga" else "FAIL")

    # Transisi Status Valid: menunggu -> diproses
    r = http_req(f"/api/v1/admin/reports/{created_id}/status", method="PATCH", data={
        "status": "diproses",
        "keterangan": "Tim teknis PDAM dan Pemda sedang menuju lokasi kebocoran pipa."
    }, headers={"Authorization": f"Bearer {admin_token}"})
    record("STA-01", "Functional Testing", "Transisi Status Valid (menunggu -> diproses)",
           "HTTP 200, status diproses",
           f"HTTP {r['status_code']}, status: {r['body'].get('status') if isinstance(r['body'], dict) else ''}",
           "PASS" if r["status_code"] == 200 and r["body"].get("status") == "diproses" else "FAIL")

    # Transisi Status Invalid: diproses -> menunggu (tidak boleh mundur)
    r = http_req(f"/api/v1/admin/reports/{created_id}/status", method="PATCH", data={
        "status": "menunggu",
        "keterangan": "Mencoba memundurkan status kembali ke menunggu."
    }, headers={"Authorization": f"Bearer {admin_token}"})
    record("STA-02", "Black Box Testing", "Transisi Status Tidak Sah (Mundur: diproses -> menunggu)",
           "HTTP 422 INVALID_TRANSITION",
           f"HTTP {r['status_code']}, code: {r['body'].get('error', {}).get('code') if isinstance(r['body'], dict) else ''}",
           "PASS" if r["status_code"] == 422 and r["body"].get("error", {}).get("code") == "INVALID_TRANSITION" else "FAIL")

    # Transisi Status Tanpa Keterangan
    r = http_req(f"/api/v1/admin/reports/{created_id}/status", method="PATCH", data={
        "status": "selesai",
        "keterangan": "   " # spasi kosong
    }, headers={"Authorization": f"Bearer {admin_token}"})
    record("STA-03", "Black Box Testing", "Ubah Status Tanpa Keterangan Tindakan",
           "HTTP 422 VALIDATION_ERROR",
           f"HTTP {r['status_code']}",
           "PASS" if r["status_code"] == 422 else "FAIL")

    # Transisi Status Valid: diproses -> selesai
    r = http_req(f"/api/v1/admin/reports/{created_id}/status", method="PATCH", data={
        "status": "selesai",
        "keterangan": "Perbaikan pipa tuntas, saluran air kembali normal dan debit stabil."
    }, headers={"Authorization": f"Bearer {admin_token}"})
    record("STA-04", "Functional Testing", "Transisi Status Valid (diproses -> selesai)",
           "HTTP 200, status selesai",
           f"HTTP {r['status_code']}, status: {r['body'].get('status') if isinstance(r['body'], dict) else ''}",
           "PASS" if r["status_code"] == 200 and r["body"].get("status") == "selesai" else "FAIL")

    # Status Selesai adalah FINAL (tidak bisa diubah lagi)
    r = http_req(f"/api/v1/admin/reports/{created_id}/status", method="PATCH", data={
        "status": "diproses",
        "keterangan": "Mencoba membuka kembali status yang sudah selesai."
    }, headers={"Authorization": f"Bearer {admin_token}"})
    record("STA-05", "Black Box Testing", "Integritas Status Selesai adalah Final",
           "HTTP 422 INVALID_TRANSITION",
           f"HTTP {r['status_code']}, code: {r['body'].get('error', {}).get('code') if isinstance(r['body'], dict) else ''}",
           "PASS" if r["status_code"] == 422 else "FAIL")

    # Warga Cek Lini Masa (Audit Trail report_updates)
    r = http_req(f"/api/v1/reports/{created_id}", headers={"Authorization": f"Bearer {warga_token}"})
    riwayat = r["body"].get("riwayat", []) if isinstance(r["body"], dict) else []
    record("INT-01", "Integration Testing", "Integritas Lini Masa Audit Trail report_updates",
           "Lini masa mencatat 3 fase (menunggu, diproses, selesai)",
           f"Jumlah riwayat: {len(riwayat)}, status_akhir: {r['body'].get('status')}",
           "PASS" if len(riwayat) == 3 and r["body"].get("status") == "selesai" else "FAIL")

    # ----------------------------------------------------
    # 6. STATIC ASSETS & FRONTEND BLACK BOX TESTING
    # ----------------------------------------------------
    static_pages = [
        ("/", "Landing Page redirect / login"),
        ("/login.html", "Halaman Login Publik"),
        ("/register.html", "Halaman Registrasi Publik"),
        ("/warga/buat-laporan.html", "Halaman Form Lapor Warga"),
        ("/warga/riwayat.html", "Halaman Riwayat Warga"),
        ("/warga/detail-laporan.html", "Halaman Detail Laporan Warga"),
        ("/warga/profil.html", "Halaman Profil Warga"),
        ("/admin/dashboard.html", "Halaman Dashboard Admin"),
        ("/admin/laporan.html", "Halaman Manajemen Laporan Admin"),
        ("/admin/detail-laporan.html", "Halaman Detail Laporan Admin"),
        ("/admin/profil.html", "Halaman Profil Admin"),
        ("/static/partials/footer.html", "Partial Footer Institusional"),
        ("/static/img/logo-pemda.png", "Logo Pemprov Lampung"),
        ("/static/img/logo-uti.png", "Logo Universitas Teknokrat Indonesia")
    ]
    for page_path, page_name in static_pages:
        r = http_req(page_path)
        record(f"UI-{page_path.replace('/', '_').replace('.', '_').strip('_')}", 
               "Black Box Testing", f"Akses Berkas Web: {page_name}",
               "HTTP 200 OK", f"HTTP {r['status_code']}",
               "PASS" if r["status_code"] == 200 else "FAIL")

    # ----------------------------------------------------
    # 7. PERFORMANCE & LOAD TESTING (50 Concurrent Simulation)
    # ----------------------------------------------------
    print("Menjalankan simulasi beban 50 concurrent requests...")
    import concurrent.futures
    latencies = []
    
    def worker():
        r = http_req("/health")
        return r["elapsed_ms"]
        
    with concurrent.futures.ThreadPoolExecutor(max_workers=50) as executor:
        futures = [executor.submit(worker) for _ in range(50)]
        for f in concurrent.futures.as_completed(futures):
            latencies.append(f.result())
            
    latencies.sort()
    p95 = latencies[int(len(latencies) * 0.95)]
    avg_lat = sum(latencies) / len(latencies)
    record("PRF-01", "Performance Testing", "Beban 50 Pengguna Bersamaan (AC-23 P95 <= 500ms)",
           f"P95 <= 500 ms",
           f"P95 = {p95:.2f} ms, Rata-rata = {avg_lat:.2f} ms (50 request simultan)",
           "PASS" if p95 <= 500 else "FAIL")

    # ----------------------------------------------------
    # 8. SECURITY & INJECTION TESTING (SQLi & XSS)
    # ----------------------------------------------------
    xss_payload = "<script>alert('XSS_ATTACK')</script>"
    sqli_payload = "'; DROP TABLE reports; --"
    body_data, ct = build_multipart({
        "kategori_id": "1",
        "deskripsi": f"Uji penetrasi keamanan dengan muatan XSS {xss_payload} dan SQLi {sqli_payload}",
        "lokasi": f"Jl. Keamanan Siber {sqli_payload}",
        "wilayah": f"Bandar Lampung {xss_payload}"
    })
    r = http_req("/api/v1/reports", method="POST", data=body_data, headers={
        "Authorization": f"Bearer {warga_token}",
        "Content-Type": ct
    })
    sqli_id = r["body"].get("id") if isinstance(r["body"], dict) else None
    
    # Verifikasi apakah database masih utuh dan data tersimpan dengan escaping
    r_check = http_req(f"/api/v1/reports/{sqli_id}", headers={"Authorization": f"Bearer {warga_token}"})
    safe_db = r_check["status_code"] == 200 and r_check["body"].get("deskripsi") is not None
    record("SEC-05", "Security Testing", "Ketahanan Terhadap SQL Injection (Parameterized Query)",
           "HTTP 201 tersimpan aman sebagai string literal, tabel tidak rusak/drop",
           f"HTTP {r['status_code']}, ID: {sqli_id}, Tabel Utuh: {safe_db}",
           "PASS" if r["status_code"] == 201 and safe_db else "FAIL")

    # ----------------------------------------------------
    # 9. DATABASE INTEGRITY TESTING
    # ----------------------------------------------------
    # Uji atomisitas sequence: nomor bertambah secara urut
    body_data1, ct1 = build_multipart({
        "kategori_id": "1",
        "deskripsi": "Laporan pengujian konsistensi urutan nomor laporan bagian 1.",
        "lokasi": "Jl. Urutan No. 1"
    })
    body_data2, ct2 = build_multipart({
        "kategori_id": "1",
        "deskripsi": "Laporan pengujian konsistensi urutan nomor laporan bagian 2.",
        "lokasi": "Jl. Urutan No. 2"
    })
    r1 = http_req("/api/v1/reports", method="POST", data=body_data1, headers={"Authorization": f"Bearer {warga_token}", "Content-Type": ct1})
    r2 = http_req("/api/v1/reports", method="POST", data=body_data2, headers={"Authorization": f"Bearer {warga_token}", "Content-Type": ct2})
    no1 = r1["body"].get("nomor", "") if isinstance(r1["body"], dict) else ""
    no2 = r2["body"].get("nomor", "") if isinstance(r2["body"], dict) else ""
    # Format WW-YYYYMMDD-NNNN
    seq1 = int(no1.split("-")[-1]) if no1.count("-") == 2 else 0
    seq2 = int(no2.split("-")[-1]) if no2.count("-") == 2 else 0
    is_sequential = (seq2 == seq1 + 1)
    record("DB-01", "Database Testing", "Penomoran Atomik Sequence (WW-YYYYMMDD-NNNN)",
           "Nomor laporan berurutan secara konsisten (n dan n+1)",
           f"Nomor 1: {no1}, Nomor 2: {no2}, Berurutan: {is_sequential}",
           "PASS" if is_sequential else "FAIL")

    # ----------------------------------------------------
    # 10. EDGE CASE & NEGATIVE TESTING
    # ----------------------------------------------------
    # Kategori ID tidak valid (9999)
    body_data_fake_cat, ct_fc = build_multipart({
        "kategori_id": "9999",
        "deskripsi": "Deskripsi laporan dengan kategori fiktif yang tidak ada.",
        "lokasi": "Jl. Kategori Palsu"
    })
    r_fc = http_req("/api/v1/reports", method="POST", data=body_data_fake_cat, headers={"Authorization": f"Bearer {warga_token}", "Content-Type": ct_fc})
    record("EDG-01", "Black Box Testing", "Lapor dengan ID Kategori Non-Eksisten (9999)",
           "HTTP 422 VALIDATION_ERROR (Kategori tidak ditemukan)",
           f"HTTP {r_fc['status_code']}, code: {r_fc['body'].get('error', {}).get('code') if isinstance(r_fc['body'], dict) else ''}",
           "PASS" if r_fc["status_code"] == 422 else "FAIL")

    # Warga coba tembak endpoint ubah status
    r_warga_status = http_req(f"/api/v1/admin/reports/{created_id}/status", method="PATCH", data={
        "status": "diproses",
        "keterangan": "Percobaan manipulasi status oleh warga biasa"
    }, headers={"Authorization": f"Bearer {warga_token}"})
    record("SEC-06", "Security Testing", "Warga Mencoba Memperbarui Status Laporan",
           "HTTP 403 FORBIDDEN",
           f"HTTP {r_warga_status['status_code']}",
           "PASS" if r_warga_status["status_code"] == 403 else "FAIL")

    # Filter laporan admin dengan keyword pencarian
    r_search = http_req("/api/v1/admin/reports?q=semburan&per_page=5", headers={"Authorization": f"Bearer {admin_token}"})
    search_matches = r_search["body"].get("data", []) if isinstance(r_search["body"], dict) else []
    record("ADM-04", "Functional Testing", "Pencarian Kata Kunci Laporan pada Admin (/admin/reports?q=...)",
           "HTTP 200, mengembalikan laporan yang relevan dengan kata kunci",
           f"HTTP {r_search['status_code']}, item_cocok: {len(search_matches)}",
           "PASS" if r_search["status_code"] == 200 and len(search_matches) >= 1 else "FAIL")

    print("\n=== RINGKASAN HASIL QA TESTING ===")
    total_tests = len(results)
    passed_tests = sum(1 for x in results if x["status"] == "PASS")
    failed_tests = sum(1 for x in results if x["status"] == "FAIL")
    print(f"Total Test Cases : {total_tests}")
    print(f"Lulus (PASS)     : {passed_tests}")
    print(f"Gagal (FAIL)     : {failed_tests}")

    with open("qa_test_report_data.json", "w", encoding="utf-8") as f:
        json.dump({
            "summary": {
                "total": total_tests,
                "passed": passed_tests,
                "failed": failed_tests,
                "rate": f"{(passed_tests/total_tests)*100:.1f}%",
                "p95_ms": round(p95, 2),
                "avg_ms": round(avg_lat, 2)
            },
            "results": results
        }, f, indent=2)

if __name__ == "__main__":
    run_tests()
