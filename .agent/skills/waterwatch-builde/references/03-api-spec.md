# API Specification – WaterWatch v1.0

> Sumber: PRD v1.0 bagian 12
> Base URL: `/api/v1`
> Format: `application/json` (kecuali upload: `multipart/form-data`)
> Auth: `Authorization: Bearer <JWT>`

---

## 1. Daftar Endpoint MVP

| Method | Endpoint | Akses | Deskripsi |
|---|---|---|---|
| `GET` | `/health` | Publik | Health check |
| `POST` | `/api/v1/auth/register` | Publik | Registrasi warga |
| `POST` | `/api/v1/auth/login` | Publik | Login, mengembalikan JWT |
| `GET` | `/api/v1/auth/me` | Login (any role) | Profil pengguna saat ini |
| `GET` | `/api/v1/categories` | Publik | Daftar kategori |
| `POST` | `/api/v1/reports` | Warga (`masyarakat`) | Buat laporan (multipart) |
| `GET` | `/api/v1/reports/mine` | Warga | Riwayat laporan milik sendiri (pagination, filter) |
| `GET` | `/api/v1/reports/{id}` | Pemilik / Admin | Detail laporan + riwayat status |
| `GET` | `/api/v1/reports/{id}/foto` | Pemilik / Admin | Unduh/tampil foto laporan |
| `GET` | `/api/v1/admin/dashboard` | Admin | Ringkasan & statistik |
| `GET` | `/api/v1/admin/reports` | Admin | Semua laporan (filter, cari, pagination) |
| `PATCH` | `/api/v1/admin/reports/{id}/status` | Admin | Ubah status + keterangan |

### Fase 2 (endpoint TIDAK diimplementasi di MVP)
| Method | Endpoint | Akses | Deskripsi |
|---|---|---|---|
| `GET` | `/api/v1/announcements` | Publik | Daftar pengumuman |
| `POST` | `/api/v1/admin/announcements` | Admin | Buat pengumuman |
| `PUT` | `/api/v1/admin/announcements/{id}` | Admin | Ubah pengumuman |
| `DELETE` | `/api/v1/admin/announcements/{id}` | Admin | Hapus pengumuman |
| `GET` | `/api/v1/admin/reports/map` | Admin | Data marker peta |

---

## 2. Kontrak Endpoint

### `GET /health`
```json
// Response 200
{ "status": "ok", "version": "0.1.0" }
```

---

### `POST /api/v1/auth/register`
```json
// Request (JSON)
{
  "nama": "Budi Santoso",
  "email": "budi@example.com",
  "password": "rahasia123"
}

// Response 201
{
  "id": "uuid-v4",
  "nama": "Budi Santoso",
  "email": "budi@example.com",
  "role": "masyarakat",
  "created_at": "2026-11-10T08:00:00Z"
}

// Error 409 – email sudah terdaftar
{
  "error": {
    "code": "CONFLICT",
    "message": "Email sudah terdaftar",
    "details": [{ "field": "email" }]
  }
}

// Error 422 – validasi gagal
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Data tidak valid",
    "details": [
      { "field": "password", "message": "Password minimal 8 karakter" }
    ]
  }
}
```

---

### `POST /api/v1/auth/login`
```json
// Request
{ "email": "budi@example.com", "password": "rahasia123" }

// Response 200
{
  "token": "<jwt>",
  "expires_in": 3600,
  "user": { "id": "uuid", "nama": "Budi Santoso", "role": "masyarakat" }
}

// Error 401 – kredensial salah (pesan generik, tidak bocorkan apakah email/password yang salah)
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Email atau password salah",
    "details": []
  }
}

// Error 429 – rate limit
{
  "error": {
    "code": "TOO_MANY_REQUESTS",
    "message": "Terlalu banyak percobaan login. Coba lagi nanti.",
    "details": []
  }
}
```

---

### `GET /api/v1/auth/me`
```json
// Response 200
{
  "id": "uuid",
  "nama": "Budi Santoso",
  "email": "budi@example.com",
  "role": "masyarakat",
  "created_at": "2026-11-10T08:00:00Z"
}
```

---

### `GET /api/v1/categories`
```json
// Response 200
{
  "data": [
    { "id": 1, "nama_kategori": "Kebocoran Pipa" },
    { "id": 2, "nama_kategori": "Air Tidak Mengalir" },
    { "id": 3, "nama_kategori": "Kualitas Air Bermasalah" },
    { "id": 4, "nama_kategori": "Lainnya" }
  ]
}
```

---

### `POST /api/v1/reports` (multipart/form-data)
```
// Request fields:
kategori_id  : integer (required)
deskripsi    : string  (required, 10–1000 karakter)
lokasi       : string  (required, alamat teks)
latitude     : float   (optional, -90..90)
longitude    : float   (optional, -180..180)
wilayah      : string  (optional)
foto         : file    (optional, JPG/PNG ≤ 5 MB)

// Response 201
{
  "id": "uuid",
  "nomor": "WW-20261110-0001",
  "status": "menunggu",
  "created_at": "2026-11-10T08:15:00Z"
}

// Error 413 – file > 5 MB
{
  "error": { "code": "PAYLOAD_TOO_LARGE", "message": "Ukuran file melebihi 5 MB", "details": [] }
}

// Error 415 – bukan JPG/PNG (berdasarkan magic bytes)
{
  "error": { "code": "UNSUPPORTED_MEDIA_TYPE", "message": "Hanya format JPG dan PNG yang didukung", "details": [] }
}

// Error 422 – validasi gagal (koordinat di luar rentang, deskripsi pendek, lokasi kosong)
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Data tidak valid",
    "details": [
      { "field": "deskripsi", "message": "Deskripsi minimal 10 karakter" },
      { "field": "latitude",  "message": "Latitude harus antara -90 dan 90" }
    ]
  }
}
```

---

### `GET /api/v1/reports/mine`
```
// Query params:
?page=1          (default 1)
?per_page=10     (default 10)
?status=menunggu (optional: menunggu|diproses|selesai)

// Response 200
{
  "data": [
    {
      "id": "uuid",
      "nomor": "WW-20261110-0001",
      "kategori": "Kebocoran Pipa",
      "deskripsi": "Pipa utama bocor...",
      "lokasi": "Jl. Contoh No. 10",
      "status": "menunggu",
      "created_at": "2026-11-10T08:15:00Z",
      "updated_at": "2026-11-10T08:15:00Z"
    }
  ],
  "meta": {
    "page": 1,
    "per_page": 10,
    "total": 3,
    "total_pages": 1
  }
}
```

---

### `GET /api/v1/reports/{id}`
```json
// Response 200 – pemilik atau admin
{
  "id": "uuid",
  "nomor": "WW-20261110-0001",
  "kategori": "Kebocoran Pipa",
  "deskripsi": "Pipa utama bocor di depan rumah, air meluap ke jalan.",
  "lokasi": "Jl. Contoh No. 10, Kelurahan X",
  "latitude": -5.4294,
  "longitude": 105.2610,
  "wilayah": "Kelurahan X",
  "foto": "/api/v1/reports/uuid/foto",
  "status": "diproses",
  "created_at": "2026-11-10T08:15:00Z",
  "updated_at": "2026-11-10T09:00:00Z",
  "riwayat": [
    {
      "status": "menunggu",
      "keterangan": "Laporan masuk",
      "updated_by_nama": null,
      "updated_at": "2026-11-10T08:15:00Z"
    },
    {
      "status": "diproses",
      "keterangan": "Laporan valid, tim perbaikan dijadwalkan.",
      "updated_by_nama": "Sari Admin",
      "updated_at": "2026-11-10T09:00:00Z"
    }
  ]
}

// Error 404 – tidak ditemukan atau bukan milik warga
{
  "error": { "code": "NOT_FOUND", "message": "Laporan tidak ditemukan", "details": [] }
}
```

---

### `GET /api/v1/reports/{id}/foto`
```
// Response 200 – Content-Type: image/jpeg atau image/png
// Header: Content-Disposition: inline; filename="<uuid>.jpg"

// Error 404 – laporan tidak ada foto
{
  "error": { "code": "NOT_FOUND", "message": "Foto tidak tersedia", "details": [] }
}
```

---

### `GET /api/v1/admin/dashboard`
```json
// Response 200
{
  "total": 125,
  "per_status": {
    "menunggu": 30,
    "diproses": 45,
    "selesai": 50
  },
  "hari_ini": 5,
  "per_kategori": [
    { "kategori": "Kebocoran Pipa",         "jumlah": 60 },
    { "kategori": "Air Tidak Mengalir",      "jumlah": 40 },
    { "kategori": "Kualitas Air Bermasalah", "jumlah": 20 },
    { "kategori": "Lainnya",                 "jumlah": 5 }
  ],
  "laporan_terbaru": [
    {
      "id": "uuid",
      "nomor": "WW-20261110-0005",
      "kategori": "Kebocoran Pipa",
      "lokasi": "Jl. Merdeka No. 1",
      "status": "menunggu",
      "created_at": "2026-11-10T10:00:00Z"
    }
  ]
}
```

---

### `GET /api/v1/admin/reports`
```
// Query params:
?page=1
?per_page=10
?status=menunggu           (optional)
?kategori_id=1             (optional)
?tanggal_dari=2026-11-01   (optional, format YYYY-MM-DD)
?tanggal_sampai=2026-11-10 (optional)
?q=bocor                   (optional, kata kunci cari di deskripsi/lokasi)

// Response 200 – sama struktur dengan /reports/mine tapi ada field user
{
  "data": [
    {
      "id": "uuid",
      "nomor": "WW-20261110-0001",
      "user_nama": "Budi Santoso",
      "kategori": "Kebocoran Pipa",
      "deskripsi": "Pipa utama bocor...",
      "lokasi": "Jl. Contoh No. 10",
      "status": "menunggu",
      "created_at": "2026-11-10T08:15:00Z",
      "updated_at": "2026-11-10T08:15:00Z"
    }
  ],
  "meta": {
    "page": 1,
    "per_page": 10,
    "total": 50,
    "total_pages": 5
  }
}
```

---

### `PATCH /api/v1/admin/reports/{id}/status`
```json
// Request
{
  "status": "diproses",
  "keterangan": "Laporan valid, tim perbaikan dijadwalkan hari ini."
}

// Response 200
{
  "id": "uuid",
  "status": "diproses",
  "updated_at": "2026-11-10T09:00:00Z"
}

// Error 422 – transisi tidak valid
{
  "error": {
    "code": "INVALID_TRANSITION",
    "message": "Transisi status tidak valid: selesai tidak bisa diubah",
    "details": [{ "field": "status", "message": "Status selesai adalah final" }]
  }
}

// Error 422 – keterangan kosong
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Keterangan wajib diisi saat memperbarui status",
    "details": [{ "field": "keterangan" }]
  }
}
```

---

## 3. Format Error Standar (PRD 12.3)

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Pesan singkat yang bisa ditampilkan ke pengguna",
    "details": [
      { "field": "nama_field", "message": "Pesan per field (opsional)" }
    ]
  }
}
```

### Kode Status HTTP

| Kode | Penggunaan |
|---|---|
| 200 | Berhasil |
| 201 | Berhasil dibuat |
| 400 | Request tidak valid |
| 401 | Belum login atau token tidak valid |
| 403 | Role tidak berhak |
| 404 | Data tidak ditemukan |
| 409 | Konflik (email sudah terdaftar) |
| 413 | File terlalu besar (> 5 MB) |
| 415 | Tipe file tidak didukung (bukan JPG/PNG) |
| 422 | Validasi gagal atau transisi status tidak valid |
| 429 | Terlalu banyak permintaan (rate limit) |
| 500 | Kesalahan server internal |

### Kode Error (field `code`)

| Code | Situasi |
|---|---|
| `VALIDATION_ERROR` | Input tidak valid, detail per field |
| `UNAUTHORIZED` | Token tidak ada / kadaluwarsa / kredensial salah |
| `FORBIDDEN` | Role tidak berhak |
| `NOT_FOUND` | Data tidak ditemukan |
| `CONFLICT` | Email sudah terdaftar |
| `PAYLOAD_TOO_LARGE` | File > 5 MB |
| `UNSUPPORTED_MEDIA_TYPE` | Bukan JPG/PNG |
| `INVALID_TRANSITION` | Transisi status tidak valid |
| `TOO_MANY_REQUESTS` | Rate limit login |
| `INTERNAL_ERROR` | Kesalahan server |

