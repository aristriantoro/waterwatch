# Panduan Deployment Online WaterWatch
**Frontend:** Vercel  
**Backend:** Render.com (Rust Axum Docker) / Railway  
**Database:** Neon.tech / Supabase (PostgreSQL 16)  

---

## 1. Persiapan Database Produksi (Neon.tech / Supabase)

Layanan yang sangat direkomendasikan adalah **Neon.tech** (gratis, tanpa kartu kredit, PostgreSQL 16 serverless).

### Langkah-langkah di Neon.tech:
1. Buka [https://neon.tech](https://neon.tech) dan daftar/login (bisa pakai GitHub).
2. Buat Project baru, beri nama: `waterwatch-db`.
3. Pilih Region: **Singapore (ap-southeast-1)** agar latensi ke Indonesia sangat rendah (< 30 ms).
4. Klik **Create Project**.
5. Simpan **Connection String** yang muncul, contoh:
   ```text
   postgresql://waterwatch_owner:AbCdEf123456@ep-cool-water-123456.ap-southeast-1.aws.neon.tech/neondb?sslmode=require
   ```
6. Buka tab **SQL Editor** di Neon Console.
7. Buka berkas `migrations/production_schema_and_seed.sql` di proyek WaterWatch, salin seluruh kodenya, tempel (*paste*) ke SQL Editor Neon, lalu klik tombol **Run**.
8. Seluruh tabel, tipe data enum, indeks, kategori, dan akun awal (Admin & Warga) berhasil dibuat dalam hitungan detik!

---

## 2. Persiapan Repositori GitHub

Seluruh kode proyek WaterWatch perlu diunggah ke repositori GitHub Anda:

1. Buka [https://github.com](https://github.com) dan buat repositori baru bernama `waterwatch`.
2. Di komputer lokal Anda (PowerShell):
   ```powershell
   cd "f:\Kuliah\Semester 5\PPSI\UAS\waterwatch"
   git remote add origin https://github.com/USERNAME-ANDA/waterwatch.git
   git branch -M main
   git push -u origin main
   ```

---

## 3. Deployment Backend Rust (Render.com)

Render.com mendukung kompilasi multi-stage `Dockerfile` secara langsung dari repositori GitHub.

### Langkah-langkah di Render.com:
1. Buka [https://render.com](https://render.com) dan daftar/login dengan GitHub Anda.
2. Klik tombol **New +** di pojok kanan atas $\rightarrow$ pilih **Web Service**.
3. Hubungkan repositori GitHub `waterwatch` yang sudah di-push tadi.
4. Konfigurasikan Web Service:
   - **Name:** `waterwatch-api`
   - **Region:** Singapore
   - **Branch:** `main`
   - **Runtime:** **Docker** (Render akan otomatis mendeteksi `Dockerfile`)
   - **Instance Type:** Free
5. Buka menu **Advanced**:
   - **Build Args (PENTING):**
     - Key: `DATABASE_URL`
     - Value: `[Connection String Neon Anda tadi]`
   - **Environment Variables:**
     - `DATABASE_URL`: `[Connection String Neon Anda]`
     - `JWT_SECRET`: `kunci_rahasia_produksi_waterwatch_sangat_panjang_dan_aman_64char`
     - `ALLOWED_ORIGINS`: `*` *(atau domain Vercel Anda)*
     - `MAX_UPLOAD_BYTES`: `5242880`
     - `UPLOAD_DIR`: `/tmp/uploads`
     - `RATE_LIMIT_BURST`: `5`
6. Klik **Create Web Service**.
7. Render akan menjalankan build Docker dan menyalakan server. Setelah selesai, Anda akan mendapatkan URL Backend, contoh:
   👉 `https://waterwatch-api.onrender.com`
8. Uji kesehatan API: buka `https://waterwatch-api.onrender.com/health` di browser. Hasilnya harus:
   ```json
   {"status":"ok","version":"0.1.0"}
   ```

---

## 4. Deployment Frontend (Vercel)

Vercel akan melayani file statis HTML/CSS/JS dan bertindak sebagai proxy aman ke Backend Render.

### Langkah-langkah di Vercel:
1. Di proyek lokal Anda, sesuaikan berkas `vercel.json`:
   Ubah `BACKEND_URL_PLACEHOLDER` menjadi domain backend Render Anda (tanpa `https://`):
   ```json
   {
     "rewrites": [
       { "source": "/api/(.*)", "destination": "https://waterwatch-api.onrender.com/api/$1" },
       { "source": "/health", "destination": "https://waterwatch-api.onrender.com/health" }
     ]
   }
   ```
2. Commit dan push perubahan:
   ```powershell
   git add vercel.json
   git commit -m "chore: update backend destination URL in vercel.json"
   git push origin main
   ```
3. Buka [https://vercel.com](https://vercel.com) dan login dengan akun GitHub Anda.
4. Klik **Add New...** $\rightarrow$ **Project**.
5. Pilih repositori `waterwatch` lalu klik **Import**.
6. Konfigurasi Proyek di Vercel:
   - **Framework Preset:** **Other**
   - **Root Directory:** `./`
   - **Build and Output Settings:** Kosongkan (karena pure static HTML/CSS/JS)
7. Klik tombol **Deploy**.
8. Dalam hitungan detik, aplikasi frontend Anda aktif dengan domain seperti:
   👉 `https://waterwatch.vercel.app`

---

## 5. Pengujian Setelah Online (Smoke Testing)

Setelah kedua layanan online, lakukan verifikasi:

1. **Akses Beranda:** Buka `https://waterwatch.vercel.app`
2. **Uji Login Admin:** Masuk ke `/login.html` dengan:
   - Email: `admin@waterwatch.id`
   - Password: `Admin@1234!`
   - Pastikan statistik dashboard muncul.
3. **Uji Pelaporan Warga:**
   - Login sebagai `warga1@example.com` (password: `Warga@1234!`).
   - Buat laporan baru dengan lampiran foto bukti.
   - Pastikan nomor laporan muncul (contoh: `WW-20261110-0001`).
4. **Uji Siklus Status:**
   - Kembali ke akun admin.
   - Buka laporan baru tersebut dan ubah status ke **Diproses** lalu **Selesai**.
