# Panduan Docker untuk WaterWatch
## Panduan Lengkap untuk Pemula Windows

---

## Apa itu Docker?

Docker adalah alat yang memungkinkan kamu menjalankan aplikasi dalam "kotak" terisolasi (disebut **container**), tanpa perlu menginstall bahasa pemrograman atau database secara manual di komputermu.

Untuk WaterWatch, kamu hanya butuh **Docker Desktop** – semua lainnya (Rust, PostgreSQL, Nginx) berjalan di dalam container.

---

## Langkah 1: Pastikan WSL 2 Sudah Aktif

Docker Desktop di Windows menggunakan WSL 2 (Windows Subsystem for Linux).

**Buka PowerShell sebagai Administrator**, ketik:
```powershell
wsl --status
```

Hasil yang benar:
```
Default Version: 2
```

Jika belum ada atau masih versi 1, jalankan:
```powershell
wsl --install
```
Lalu **restart komputer**.

---

## Langkah 2: Install Docker Desktop

1. Buka browser, kunjungi: **https://www.docker.com/products/docker-desktop/**
2. Klik tombol **"Download for Windows"**
3. Buka file `.exe` yang diunduh, ikuti wizard instalasi
4. Saat ditanya, centang opsi **"Use WSL 2 instead of Hyper-V"**
5. Klik **Install** dan tunggu hingga selesai
6. **Restart komputer** jika diminta

---

## Langkah 3: Verifikasi Docker Berjalan

Setelah restart, buka **Docker Desktop** dari Start Menu. Tunggu hingga status di pojok kiri bawah berubah menjadi **hijau** dan bertuliskan "Engine running".

Lalu buka PowerShell biasa (tidak perlu Administrator), ketik:
```powershell
docker ps
```

Hasil yang benar (tidak ada error):
```
CONTAINER ID   IMAGE   COMMAND   CREATED   STATUS   PORTS   NAMES
```

---

## Menjalankan WaterWatch

Semua perintah dijalankan dari folder proyek. Buka PowerShell, pindah ke folder:
```powershell
Set-Location "f:\Kuliah\Semester 5\PPSI\UAS\waterwatch"
```

### A. Build Pertama Kali (10–20 menit)
```powershell
docker compose build
```
Ini akan mengunduh Rust, kompilasi kode, dan membuat image. **Normal jika lama** saat pertama kali.

Hasil yang benar di akhir:
```
✔ app  Built
```

### B. Jalankan Migrasi Database
```powershell
docker compose up -d postgres
docker compose run --rm migrate
```

Hasil yang benar:
```
1/u users (...)
2/u seed_categories (...)
```

### C. Isi Data Simulasi (Seed)
```powershell
docker compose run --rm app /app/seed
```

Hasil yang benar:
```
INFO Admin seeded: admin@waterwatch.id
INFO Warga seeded: warga1@example.com
...
INFO ✅ Seed selesai!
```

### D. Jalankan Semua Layanan
```powershell
docker compose up
```

Tunggu hingga muncul:
```
ww-app    | INFO Listening on 0.0.0.0:8080
```

Lalu buka browser: **http://localhost**

### E. Hentikan Aplikasi
Tekan `Ctrl+C` di PowerShell, lalu:
```powershell
docker compose down
```

---

## Menjalankan Integration Tests

Pastikan stack sudah berjalan (`docker compose up -d`), lalu:
```powershell
docker compose run --rm -e TEST_SERVER_URL=http://app:8080 app cargo test 2>&1
```

---

## Perintah Berguna

| Perintah | Fungsi |
|---|---|
| `docker compose up` | Jalankan semua layanan (foreground) |
| `docker compose up -d` | Jalankan semua layanan (background) |
| `docker compose down` | Hentikan dan hapus container |
| `docker compose down -v` | Hentikan dan hapus container + data database |
| `docker compose logs app` | Lihat log aplikasi Rust |
| `docker compose logs -f app` | Lihat log real-time |
| `docker compose ps` | Status semua container |
| `docker compose build` | Build ulang image (setelah ubah kode) |

---

## Akun Default (dari Seed)

| Role | Email | Password |
|---|---|---|
| Admin | `admin@waterwatch.id` | `Admin@1234!` |
| Admin 2 | `admin2@waterwatch.id` | `Admin@1234!` |
| Warga | `warga1@example.com` | `Warga@1234!` |
| Warga | `warga2@example.com` | `Warga@1234!` |

---

## Troubleshooting

### "Cannot connect to Docker daemon"
→ Buka **Docker Desktop** dari Start Menu, tunggu status berubah hijau.

### Build gagal: "network timeout" atau "connection refused"
→ Pastikan koneksi internet aktif. Coba ulang: `docker compose build`

### Port 80 sudah dipakai
→ Ubah di `docker-compose.yml`: `"80:80"` → `"8080:80"`, lalu akses di `http://localhost:8080`

### Database error saat seed
→ Pastikan migrasi sudah dijalankan dulu: `docker compose run --rm migrate`

### Reset total (mulai dari awal)
```powershell
docker compose down -v   # hapus semua container dan data
docker compose build     # build ulang
docker compose up -d postgres
docker compose run --rm migrate
docker compose run --rm app /app/seed
docker compose up
```

