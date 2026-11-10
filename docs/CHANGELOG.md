# CHANGELOG – WaterWatch

Semua perubahan penting dicatat di sini. Format: [Versi/Fase] – Tanggal.

---

## [Fase 0] – 2026-11-10 · Persiapan & Infrastruktur

### Ditambahkan
- **Referensi proyek** (`.agent/skills/waterwatch-builde/references/`):
  - `01-prd-ringkas.md` – ringkasan persona, FR-01..27, AC-01..24, NFR
  - `02-database.sql` – skema lengkap + seed kategori
  - `03-api-spec.md` – kontrak JSON semua endpoint
  - `04-design-system.md` – token warna, tipografi, komponen UI
  - `05-ui-pages.md` – spesifikasi 11 halaman
  - `06-backend-guide.md` – panduan arsitektur Rust
  - `07-frontend-guide.md` – panduan JS vanilla + Bootstrap 5
  - `08-testing-checklist.md` – checklist AC-01..AC-24
- **Assets**: `tokens.css`, `footer.html`
- **Docs**: `docs/DECISIONS.md` (D-01..D-11, Q-01..Q-04 dijawab)
- **Infrastruktur Docker**:
  - `Dockerfile` (multi-stage: rust:1.80-slim builder → debian:bookworm-slim runner)
  - `docker-compose.yml` (services: postgres, migrate, app, nginx)
  - `nginx/nginx.conf` (security headers, proxy, static files)
- **Migrasi database**:
  - `migrations/20261110000001_init.sql` (semua tabel: users, categories, report_number_seq, reports, report_updates, announcements + indexes)
  - `migrations/20261110000002_seed_categories.sql` (4 kategori)
- **Konfigurasi**: `Cargo.toml`, `.env`, `.env.example`, `.gitignore`

---

## [Fase 1] – 2026-11-10 · Fondasi Backend

### Ditambahkan
- **Backend Rust** – arsitektur berlapis routes → handlers → services → repositories:
  - `src/config.rs` – konfigurasi via environment variables
  - `src/errors.rs` – AppError unified dengan format `{"error":{"code","message","details"}}`
  - `src/models/` – User, Claims, ReportStatus, Report, ReportSummary, ReportSummaryAdmin, ReportDetail, Category, ReportUpdate
  - `src/middleware/auth.rs` – JWT extractor (`Claims`) via `FromRequestParts`
  - `src/middleware/role_guard.rs` – `AdminClaims` extractor (cek role = admin)
  - `src/repositories/` – user, category, report (semua query terparameter SQLx)
  - `src/services/` – auth (Argon2id hash, JWT sign), reports (validasi + foto), admin (dashboard, list, update status)
  - `src/handlers/` – auth, categories, reports, admin
  - `src/routes/` – semua route terdaftar dengan middleware tepat
  - `src/main.rs` – AppState, CORS terbatas, security headers, /health endpoint
  - `src/bin/seed.rs` – binary seed: 2 admin, 12 warga, 55 laporan

### Keamanan yang diimplementasi
- Password: Argon2id (argon2 crate)
- JWT: HS256, exp 1 jam, validasi di semua protected route
- Login gagal: selalu 401 dengan pesan generik
- Warga akses laporan orang lain: 404 (bukan 403)
- Admin-only: 403 jika bukan admin
- Upload foto: validasi magic bytes (infer crate), max 5MB, nama UUID
- Rate limit login: tower_governor 5 req/60 detik per IP
- SQL: semua query via SQLx parameterized (tidak ada string concat)
- Security headers: X-Content-Type-Options, X-Frame-Options, Referrer-Policy

---

## [Fase 2 & 3] – 2026-11-10 · Frontend & Fitur Lengkap

### Ditambahkan
- **Static files**:
  - `static/css/tokens.css`, `static/css/app.css`
  - `static/js/api.js` – HTTP client dengan JWT auto-attach
  - `static/js/utils.js` – escapeHtml, formatTanggal, showToast, setTextSafe, statusBadge
  - `static/js/guard.js` – requireAuth, requireAdmin, requireWarga
  - `static/js/layout.js` – load footer partial
  - `static/partials/footer.html` – footer Pemda Lampung + UTI
- **Halaman Publik**:
  - `static/index.html` – landing page redirect
  - `static/login.html` – form login dengan validasi & rate limit feedback
  - `static/register.html` – form registrasi warga
  - `static/404.html` – halaman 404
  - `static/50x.html` – halaman error 500
- **Halaman Warga** (semua dengan 4 state: loading/empty/error/success):
  - `static/warga/riwayat.html` – daftar laporan dengan pagination & filter status
  - `static/warga/buat-laporan.html` – form laporan multipart + upload foto
  - `static/warga/detail-laporan.html` – detail laporan + riwayat perubahan status
  - `static/warga/profil.html` – profil warga
- **Halaman Admin** (semua dengan 4 state):
  - `static/admin/dashboard.html` – statistik + Chart.js + laporan terbaru
  - `static/admin/laporan.html` – daftar laporan + filter + pagination
  - `static/admin/detail-laporan.html` – detail + form ubah status
  - `static/admin/profil.html` – profil admin

### Aturan UI yang diterapkan
- Semua data user via `textContent` / `setTextSafe()` – tidak ada `innerHTML` dengan data user
- Status badge selalu: ikon + warna + teks
- Touch target ≥ 44×44px
- Bahasa Indonesia di seluruh UI
- Footer di semua halaman via `layout.js`
- Bottom nav mobile, sidebar 256px desktop

---

## [Bugfix & Penyempurnaan] – 2026-11-10

### Diperbaiki
- Hapus import `use axum::extract::State;` yang tidak terpakai di `src/middleware/auth.rs`
- Bersihkan import yang tidak terpakai di `src/services/reports.rs` (`AdminReportQuery`, `ReportSummaryAdmin`, dll.)
- Tambah field `user_nama: Option<String>` ke `ReportDetail` dan JOIN users di query detail
- Tambah derive `Copy` ke enum `ReportStatus` untuk mengatasi compile move error
- Dukungan `ARG DATABASE_URL` pada Dockerfile multi-stage untuk kompilasi offline SQLx macros
- Konfigurasi `RATE_LIMIT_BURST` dinamis melalui environment variable (default 5, 1000 saat test)
- Container service `test` khusus berbasis multi-stage builder untuk menjalankan `cargo test` tanpa install Rust lokal
- Migrasi database otomatis menggunakan container postgres-alpine berbasis script shell
- Seluruh pengujian integrasi (`auth_test`, `reports_test`, `admin_test`) berhasil 100% (ok)


