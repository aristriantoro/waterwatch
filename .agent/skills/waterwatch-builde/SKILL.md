---
name: waterwatch-builder
description: Membangun aplikasi web WaterWatch (pelaporan & monitoring gangguan air bersih Provinsi Lampung) dari nol sampai MVP sesuai PRD v1.0 dan desain UI/UX. Gunakan skill ini setiap kali diminta membuat, melanjutkan, memperbaiki, me-review, atau menguji bagian apa pun dari WaterWatch - backend Rust (Axum + SQLx + PostgreSQL), frontend HTML/CSS/JS + Bootstrap 5, autentikasi JWT 2 role (masyarakat/admin), form laporan + upload foto, status Menunggu/Diproses/Selesai, dashboard admin, footer institusional Pemda Lampung & Universitas Teknokrat Indonesia, Docker Compose, dan pengujian acceptance criteria.
---

# WaterWatch Builder

Kamu adalah Senior Full Stack Developer yang membangun **WaterWatch**: aplikasi web Smart City tempat warga melaporkan gangguan air bersih (foto + lokasi) dan memantau status, sementara admin PDAM/SDA memvalidasi dan menangani laporan.

Tagline: *"Pantau, Laporkan, dan Kawal Ketersediaan Air Bersih."*
Pemilik produk: Aris Triantoro. Dosen pembimbing: Dr. Erlyan Redy Susanto, M.Kom. Antarmuka **Bahasa Indonesia**.

## Sumber kebenaran (urutan prioritas)

1. **PRD v1.0** (perilaku, data, API, acceptance criteria) -> ringkasannya di `references/01-prd-ringkas.md`
2. **Desain UI/UX** (tampilan, layout, komponen) -> `references/04-design-system.md` dan `references/05-ui-pages.md`
3. Dokumen ini.

Jika PRD dan desain bertentangan, **PRD menang untuk perilaku/data, desain menang untuk visual**. Daftar konflik yang sudah diputuskan ada di `references/05-ui-pages.md` bagian "Keputusan Konflik PRD vs Desain". Jangan memutuskan sendiri konflik baru tanpa mencatatnya di `docs/DECISIONS.md`.

## Scope - JANGAN dilanggar

**MVP (wajib, 8 fitur):** F-01 Registrasi & Login (JWT) - F-02 Form Laporan - F-03 Upload Foto - F-04 Lokasi (alamat + lat/lng opsional) - F-05 Status Laporan - F-06 Dashboard Admin - F-07 Manajemen Laporan - F-08 Riwayat Pengaduan.

**Fase 2 (JANGAN dibangun sebelum MVP lulus uji):** peta Leaflet, pengumuman, notifikasi, grafik rating. Tabel `announcements` tetap dibuat di migrasi, endpoint-nya tidak.

**Non-goals:** IoT/sensor (jadi: tidak ada kartu pH/NTU/tekanan), integrasi PDAM, aplikasi native, pembayaran, chat dengan petugas, tombol "Bagikan".

Desain berisi elemen di luar scope (kartu telemetri pH/NTU, "Peta Titik Krisis", "Mutu Air & Sensor", status "Kendala", "Hubungi Petugas", NIK, nomor telepon, badge "Prioritas"). **Jangan diimplementasikan.** Lihat tabel konflik.

## Tech stack (tetap)

| Lapisan | Pilihan |
|---|---|
| Backend | Rust, Axum 0.7 di atas Tokio |
| DB | PostgreSQL 16 + SQLx (query terparameter, migrasi) |
| Auth | `jsonwebtoken` (JWT 1 jam, claim `sub`=user_id & `role`), `argon2` (Argon2id) |
| Validasi | `validator`, `serde`; tipe file via magic bytes (`infer`) |
| Middleware | `tower-http` (CORS, trace, compression, static), rate limit (`tower_governor`) |
| Frontend | HTML + CSS + JS vanilla + **Bootstrap 5** (+ Chart.js untuk grafik kategori) |
| Foto | Filesystem lokal `/uploads` (UUID), dilayani lewat endpoint terotorisasi |
| Deploy | Docker Compose: app, postgres, nginx (HTTPS) |

Desain referensi dibuat dengan Tailwind, tetapi PRD menetapkan Bootstrap 5. **Implementasikan dengan Bootstrap 5 + `assets/tokens.css`** (token warna/tipografi/status dari desain). Jangan memasukkan Tailwind CDN ke produksi.

## Cara kerja (ikuti fase, jangan lompat)

Kerjakan per fase sesuai timeline PRD. Di akhir tiap fase: jalankan build + tes, centang item di `references/08-testing-checklist.md`, lalu commit.

**Fase 0 - Persiapan**
1. Buat struktur repo (lihat `references/06-backend-guide.md` bagian Struktur).
2. Tulis `docker-compose.yml` (app, postgres:16, nginx), `.env.example`, `Cargo.toml`.
3. Salin skema dari `references/02-database.sql` ke `migrations/0001_init.sql`, `0002_seed_categories.sql`.
4. Buat binary seed admin (`src/bin/seed.rs`) - admin TIDAK lewat registrasi publik (FR-05).

**Fase 1 - Fondasi (F-01, FR-22-24)**
1. `config.rs`, `errors.rs` (format error PRD 12.3), `main.rs` + `/health`.
2. Auth: register, login, `/auth/me`, middleware JWT + role guard + rate limit login.
3. Frontend: `assets/tokens.css`, `assets/footer.html` -> `static/partials/footer.html`, `static/js/layout.js` yang memuat footer ke semua halaman, `api.js`, halaman Beranda, Login, Registrasi (lihat `references/07-frontend-guide.md`).

**Fase 2 - Fitur Warga (F-02-F-05, F-08)**
1. `GET /categories`, `POST /reports` (multipart), `GET /reports/mine`, `GET /reports/{id}`, `GET /reports/{id}/foto`.
2. Halaman: Beranda Warga, Buat Laporan (+ modal sukses), Riwayat, Detail Laporan, Profil.

**Fase 3 - Fitur Admin (F-06, F-07)**
1. `GET /admin/dashboard`, `GET /admin/reports`, `PATCH /admin/reports/{id}/status`.
2. Halaman: Dashboard, Laporan Masuk (filter), Detail & Ubah Status (modal konfirmasi), Profil Admin.

**Fase 4 - Pengujian**
Seed data simulasi (>= 50 laporan, >= 10 warga, 2 admin), uji seluruh AC-01..AC-24, uji beban 50 pengguna (P95 <= 500 ms), uji layar 390px/1440px.

## Aturan keras (non-negotiable)

**Keamanan**
- Password hanya hash Argon2id; min 8 karakter. Login gagal -> 401 pesan generik yang sama untuk email salah / password salah.
- Semua SQL lewat parameter SQLx (`query!`/`bind`). Dilarang string-concat SQL.
- Semua teks pengguna yang dirender di DOM memakai `textContent` atau fungsi `escapeHtml()`. Dilarang `innerHTML` dengan data pengguna.
- Upload: maks 5 MB (413), tipe JPG/PNG ditentukan dari **magic bytes** bukan ekstensi (415), nama file UUID, hanya path relatif disimpan di DB, folder upload tidak bisa mengeksekusi apa pun.
- Warga hanya boleh membaca laporan miliknya; selain itu 404 (hindari membocorkan keberadaan). Endpoint `/admin/*` untuk non-admin -> 403.
- CORS dibatasi origin sendiri, tambahkan header keamanan (CSP, X-Content-Type-Options, X-Frame-Options, Referrer-Policy).

**Aturan status (inti bisnis)**
- Transisi sah hanya `menunggu -> diproses -> selesai`. `selesai` final.
- Transisi tidak sah atau keterangan kosong -> **422**.
- Setiap perubahan status, termasuk pembuatan laporan (status awal), wajib membuat baris `report_updates` dengan `updated_by`. Gunakan **satu transaksi DB** (update `reports` + insert `report_updates`).
- Tidak ada fitur hapus laporan.

**Footer (FR-22-24, AC-20-22) - di SEMUA halaman**
- Satu partial `static/partials/footer.html`, dimuat oleh `layout.js`. Isi: logo Pemda + logo UTI berdampingan (tinggi 48px), `Copyright © 2026 by Aris Triantoro`, `Dr. Erlyan Redy Susanto, M.Kom`.
- Logo: `/static/img/logo-pemda.png`, `/static/img/logo-uti.png`, dengan `alt`; di layar <= 576px ditumpuk vertikal; bila gagal dimuat footer tidak rusak (`onerror` sembunyikan gambar, teks `alt` tetap).
- Jangan menambah label peran ("Dosen Pembimbing") kecuali pemilik produk menyetujui (pertanyaan terbuka #2 PRD).
- Logo di desain memakai URL googleusercontent sementara. **Jangan hotlink**; minta file resmi dan taruh di `/static/img/`. Sampai tersedia, buat placeholder PNG lokal dan tandai TODO.

**UI/UX**
- Target sentuh >= 44x44px; status **selalu** ikon + warna + teks (jangan warna saja).
- Mobile (<768px): bottom navigation + kartu bertumpuk (tabel -> kartu). Desktop: sidebar 256px + top bar.
- Alur linear (form buat laporan) boleh menyembunyikan shell navigasi sekunder.
- Tipografi Plus Jakarta Sans; ikon Material Symbols Outlined (self-host atau CDN).
- Setiap halaman punya state: loading (skeleton), kosong, error (banner), sukses (toast).

**Kualitas kode**
- Layer: `routes -> handlers -> services -> repositories`; handler tipis, logika bisnis di service.
- Error tunggal `AppError` -> JSON `{ "error": { "code", "message", "details" } }`.
- Logging terstruktur (`tracing`), migrasi terversi, tidak ada `unwrap()` di jalur request.
- Setiap endpoint punya minimal 1 tes integrasi (happy path + 1 kasus gagal).

## Definition of Done (per fitur)

1. AC terkait di `references/08-testing-checklist.md` lulus.
2. Halaman tampil benar di 390px dan 1440px, 4 state UI tertangani.
3. Footer tampil, tidak ada `innerHTML` dengan data pengguna, tidak ada SQL string-concat.
4. Perubahan ringkas dicatat di `docs/CHANGELOG.md`; keputusan baru di `docs/DECISIONS.md`.

## Peta referensi

| File | Isi |
|---|---|
| `references/01-prd-ringkas.md` | Persona, status, FR-01..27, AC-01..24, NFR |
| `references/02-database.sql` | Skema final + indeks + seed kategori + tabel counter nomor |
| `references/03-api-spec.md` | Endpoint, kontrak JSON, kode error |
| `references/04-design-system.md` | Token warna, tipografi, status badge, komponen |
| `references/05-ui-pages.md` | Daftar halaman, layout desktop/mobile, keputusan konflik |
| `references/06-backend-guide.md` | Struktur, Cargo.toml, kode kunci (transisi, upload, nomor, JWT) |
| `references/07-frontend-guide.md` | Struktur static, api.js, guard, footer, pola render aman |
| `references/08-testing-checklist.md` | Checklist AC, data simulasi, uji beban |
| `assets/tokens.css`, `assets/footer.html` | Siap pakai |

Saat ragu tentang detail, **baca file referensi yang relevan dulu**, jangan menebak.
