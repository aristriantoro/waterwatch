# PRD Ringkas – WaterWatch v1.0

> Sumber: PRD_WaterWatch.pdf v1.0, 10 November 2026
> Pemilik produk: Aris Triantoro | Dosen pembimbing: Dr. Erlyan Redy Susanto, M.Kom

---

## 1. Persona

| Persona | Role | Deskripsi |
|---|---|---|
| **Budi** (34 th) | Masyarakat / Warga | Menemukan pipa bocor, ingin melapor cepat lewat ponsel tanpa datang ke kantor, dan ingin tahu kapan diperbaiki. |
| **Sari** (29 th) | Admin / Pengelola | Menerima banyak pengaduan dari berbagai kanal. Butuh daftar laporan yang terfilter, bukti foto, lokasi, dan cara cepat memperbarui status serta melihat statistik. |

---

## 2. Status & Transisi

```
menunggu ──► diproses ──► selesai (final)
```

| Status | Keterangan | Transisi Valid |
|---|---|---|
| `menunggu` | Laporan baru, belum ditangani | → `diproses` |
| `diproses` | Sudah diverifikasi dan sedang ditangani | → `selesai` |
| `selesai` | Gangguan telah ditangani (FINAL, tidak bisa diubah lagi) | — |

**Aturan:**
- Transisi tidak sah → HTTP 422
- Keterangan kosong → HTTP 422
- Setiap perubahan status (termasuk pembuatan laporan) WAJIB membuat baris `report_updates` dalam SATU transaksi DB

---

## 3. Functional Requirements (FR-01..FR-27)

### 8.1 Autentikasi dan Otorisasi (F-01)
- **FR-01** Registrasi dengan nama, email unik, password min 8 karakter. Role default: `masyarakat`.
- **FR-02** Password disimpan sebagai hash Argon2id, tidak pernah teks asli.
- **FR-03** Login menghasilkan JWT (1 jam) berisi `user_id` dan `role`.
- **FR-04** Endpoint admin hanya role `admin`; pelanggaran → HTTP 403.
- **FR-05** Akun admin dibuat via seed database, bukan registrasi publik.
- **FR-06** Rate limiting pada endpoint login (mencegah brute force).

### 8.2 Pelaporan (F-02, F-03, F-04)
- **FR-07** Form wajib: `kategori_id`, `deskripsi` (10–1000 karakter), `lokasi` (alamat teks). Koordinat lat/lng opsional.
- **FR-08** Kategori awal: Kebocoran Pipa, Air Tidak Mengalir, Kualitas Air Bermasalah, Lainnya.
- **FR-09** Foto opsional, format JPG/PNG, maks 5 MB; validasi tipe via **magic bytes** (bukan ekstensi).
- **FR-10** File disimpan dengan nama UUID, hanya path relatif di DB.
- **FR-11** Laporan baru otomatis status `menunggu` dan 1 entri `report_updates`.
- **FR-12** Nomor laporan unik, format `WW-YYYYMMDD-NNNN` (counter harian, direset tiap hari).

### 8.3 Status dan Riwayat (F-05, F-08)
- **FR-13** Warga hanya bisa melihat laporan miliknya sendiri.
- **FR-14** Detail laporan: status terkini + lini masa perubahan status.
- **FR-15** Riwayat: pagination default 10/halaman, filter status.

### 8.4 Dashboard Admin (F-06)
- **FR-16** Dashboard: total laporan, jumlah per status, jumlah laporan hari ini, 10 laporan terbaru.
- **FR-17** Statistik per kategori (tabel atau grafik Chart.js).

### 8.5 Manajemen Laporan (F-07)
- **FR-18** Admin lihat semua laporan: filter status, kategori, rentang tanggal, cari kata kunci.
- **FR-19** Admin buka detail laporan + foto + lokasi.
- **FR-20** Admin perbarui status sesuai aturan transisi, wajib mengisi keterangan.
- **FR-21** Transisi tidak valid → HTTP 422 + pesan jelas.

### 8.6 Footer (FR-22..FR-24)
- **FR-22** Semua halaman (publik, warga, admin) tampilkan footer: logo Pemda, logo UTI, "Copyright © 2026 by Aris Triantoro", "Dr. Erlyan Redy Susanto, M.Kom".
- **FR-23** Footer dari 1 partial bersama (`/static/partials/footer.html`).
- **FR-24** Logo di `/static/img/logo-pemda.png` & `/static/img/logo-uti.png`, responsif (vertikal ≤576px).

### 8.7 Fitur Fase 2 (JANGAN diimplementasi di MVP)
- **FR-25** Peta Leaflet.js + OpenStreetMap dengan marker laporan.
- **FR-26** Admin kelola pengumuman per wilayah; warga lihat tanpa login.
- **FR-27** Grafik per kategori dan wilayah.

---

## 4. Acceptance Criteria (AC-01..AC-24)

### F-01 Registrasi & Login
- **AC-01** Email belum terdaftar + data valid → akun dibuat, role `masyarakat`, password sebagai hash.
- **AC-02** Email sudah terdaftar → 409.
- **AC-03** Kredensial benar → JWT dikembalikan, diarahkan sesuai role.
- **AC-04** Kredensial salah → 401 pesan generik (tidak bocorkan email mana yang ada).
- **AC-05** Role masyarakat akses `/admin/*` → 403.

### F-02..F-04 Pelaporan
- **AC-06** Warga login + data valid → laporan tersimpan status `menunggu`, nomor unik, 1 entri `report_updates`.
- **AC-07** Deskripsi < 10 karakter atau lokasi kosong → 422 + pesan per field.
- **AC-08** Foto JPG/PNG ≤ 5 MB → file tersimpan nama UUID, tampil di detail.
- **AC-09** File bukan gambar atau > 5 MB → 415 atau 413, laporan tidak tersimpan.
- **AC-10** Koordinat di luar rentang valid → 422.

### F-05 & F-08 Status & Riwayat
- **AC-11** Riwayat warga → hanya laporan miliknya, terurut terbaru, pagination.
- **AC-12** Warga buka laporan milik orang lain → 403 atau 404.
- **AC-13** Admin ubah status → warga buka detail: status terbaru + lini masa tampil.

### F-06 Dashboard Admin
- **AC-14** Ada data → total, per status, hari ini, 10 terbaru akurat.
- **AC-15** Belum ada data → nilai 0, pesan "Belum ada laporan", tanpa error.

### F-07 Manajemen Laporan
- **AC-16** Filter status/kategori/tanggal → hanya data sesuai tampil.
- **AC-17** Laporan `menunggu` → ubah ke `diproses` + keterangan → status berubah, `report_updates` bertambah dengan `updated_by`.
- **AC-18** Laporan `selesai` → ubah status → 422.
- **AC-19** Ubah status tanpa keterangan → 422.

### Footer & Lintas Fitur
- **AC-20** Semua halaman → footer dengan logo Pemda, logo UTI, copyright, nama dosen.
- **AC-21** Layar ≤576px → footer terbaca, logo tidak terpotong/menimpa.
- **AC-22** File logo gagal → teks alt tampil, footer tidak rusak.
- **AC-23** 50 pengguna bersamaan → P95 API ≤ 500 ms.
- **AC-24** Input HTML script / SQL injection → di-escape/ditolak, tidak dieksekusi.

---

## 5. Non-Functional Requirements (NFR)

| Kategori | Persyaratan |
|---|---|
| Kinerja | P95 API ≤ 500 ms pada beban ≥ 50 pengguna bersamaan |
| Keamanan | HTTPS produksi, Argon2id, JWT, SQLx terparameter, CORS terbatas, header keamanan, proteksi XSS & SQL injection |
| Privasi | Data minimum (nama, email); laporan hanya pemilik & admin |
| Ketersediaan | Target 99% selama masa uji/demo |
| Kompatibilitas | Chrome, Firefox, Edge, Safari versi terbaru; responsif ponsel |
| Aksesibilitas | Kontras memadai, label form, teks alt gambar |
| Maintainability | Kode modular, migrasi terversi, logging terstruktur |
| Bahasa | Antarmuka Bahasa Indonesia |

---

## 6. Fitur MVP (8 Fitur Wajib)

| ID | Fitur | User Stories |
|---|---|---|
| F-01 | Registrasi dan Login (JWT) | US-01, US-10 |
| F-02 | Form Pelaporan Gangguan | US-02 |
| F-03 | Upload Foto Bukti | US-03 |
| F-04 | Lokasi Gangguan (alamat + lat/lng opsional) | US-04 |
| F-05 | Status Laporan | US-05 |
| F-06 | Dashboard Admin | US-11 |
| F-07 | Manajemen Laporan (filter, ubah status) | US-12, US-13 |
| F-08 | Riwayat Pengaduan | US-06 |

---

## 7. Non-Goals (di luar scope MVP)

- IoT / sensor deteksi kebocoran
- Integrasi sistem internal PDAM
- Aplikasi mobile native
- Pembayaran / tagihan air
- Chat dengan petugas
- Tombol "Bagikan"
- Kartu telemetri (pH, NTU, tekanan)
- "Peta Titik Krisis" (Fase 2)
- Status "Kendala" (tidak ada di PRD)
- NIK / nomor telepon
- Badge "Prioritas"
- Rating (Fase 2)
- Notifikasi push (Fase 2)
- Fitur lupa password (ditunda)

