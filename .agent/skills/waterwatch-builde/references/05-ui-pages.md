# UI Pages – WaterWatch v1.0

> **Catatan**: File desain UI/UX (Figma) tidak tersedia.
> Spesifikasi halaman ini diturunkan dari PRD v1.0 bagian 13 dan aturan UI di SKILL.md.
> Asumsi desain dicatat di `docs/DECISIONS.md`.

---

## 1. Daftar Halaman

| ID | Halaman | File HTML | Role | Navigasi |
|---|---|---|---|---|
| P-01 | Beranda (publik) | `static/index.html` | Publik | — |
| P-02 | Login | `static/login.html` | Publik | — |
| P-03 | Registrasi | `static/register.html` | Publik | — |
| P-04 | Beranda Warga / Profil | `static/warga/profil.html` | Warga | Bottom nav |
| P-05 | Buat Laporan | `static/warga/buat-laporan.html` | Warga | Linear (tanpa nav) |
| P-06 | Riwayat Laporan | `static/warga/riwayat.html` | Warga | Bottom nav |
| P-07 | Detail Laporan (Warga) | `static/warga/detail-laporan.html` | Warga | Back button |
| P-08 | Dashboard Admin | `static/admin/dashboard.html` | Admin | Sidebar |
| P-09 | Daftar Laporan Admin | `static/admin/laporan.html` | Admin | Sidebar |
| P-10 | Detail Laporan Admin | `static/admin/detail-laporan.html` | Admin | Sidebar |
| P-11 | Profil Admin | `static/admin/profil.html` | Admin | Sidebar |

---

## 2. Layout Shell

### Shell Warga (Mobile)
```
┌─────────────────────────┐
│       Top Bar            │  (nama pengguna kanan, 56px)
├─────────────────────────┤
│                          │
│       KONTEN             │
│                          │
├─────────────────────────┤
│  🏠  📋  ➕  👤         │  Bottom Nav (60px fixed)
└─────────────────────────┘
```
Bottom nav items: Beranda | Riwayat | Buat Laporan | Profil

### Shell Admin (Desktop ≥768px)
```
┌───────┬─────────────────────────┐
│       │        Top Bar           │
│       ├─────────────────────────┤
│ Side  │                          │
│ bar   │       KONTEN             │
│ 256px │                          │
│       │                          │
├───────┴─────────────────────────┤
│            Footer                │
└──────────────────────────────────┘
```
Sidebar items: Dashboard | Laporan Masuk | Profil | (Keluar)

### Shell Admin (Mobile)
- Sidebar collapse jadi hamburger menu
- Bottom nav admin: Dashboard | Laporan | Profil

---

## 3. Spesifikasi Per Halaman

### P-01 – Beranda (Publik)
**URL**: `/`
**Isi**:
- Hero section: tagline "Pantau, Laporkan, dan Kawal Ketersediaan Air Bersih."
- Statistik ringkas (total laporan, terselesaikan) – dari API atau hardcoded untuk demo
- Tombol CTA: "Buat Laporan" → redirect login jika belum login
- Tombol: "Login" dan "Daftar" di navbar
- Footer

**States**:
- Loading: skeleton counter
- Sukses: angka statistik tampil
- Error: banner "Tidak dapat memuat statistik"

---

### P-02 – Login
**URL**: `/login.html`
**Isi**:
- Form: email (required), password (required)
- Tombol submit "Masuk"
- Link "Belum punya akun? Daftar di sini"
- Footer

**Perilaku**:
- Submit → `POST /api/v1/auth/login`
- Sukses: simpan token ke `localStorage`, redirect ke `/warga/riwayat.html` (masyarakat) atau `/admin/dashboard.html` (admin)
- Gagal 401: tampilkan pesan generik di bawah form
- Gagal 429: tampilkan pesan rate limit

**States**: Loading (spinner di tombol), Error (alert merah di atas form), Sukses (redirect)

---

### P-03 – Registrasi
**URL**: `/register.html`
**Isi**:
- Form: nama (required), email (required), password (required, min 8)
- Tombol submit "Daftar"
- Link "Sudah punya akun? Login"
- Footer

**Perilaku**:
- Submit → `POST /api/v1/auth/register`
- Sukses: redirect ke login dengan toast "Registrasi berhasil, silakan login"
- Gagal 409: "Email sudah terdaftar"
- Gagal 422: tampilkan error per field

---

### P-04 – Profil Warga
**URL**: `/warga/profil.html` (guard: wajib login, role masyarakat)
**Isi**:
- Nama, email, tanggal bergabung (dari `GET /api/v1/auth/me`)
- Tombol "Keluar" (hapus token, redirect ke `/`)

---

### P-05 – Buat Laporan
**URL**: `/warga/buat-laporan.html` (guard: wajib login, role masyarakat)
**Layout**: Linear – tanpa bottom nav, hanya header dengan tombol "✕ Batal"

**Form fields**:
1. Kategori (select, required) – dari `GET /api/v1/categories`
2. Deskripsi (textarea, 10–1000 karakter, required, tampilkan counter karakter)
3. Alamat Lokasi (input text, required)
4. Latitude (number, optional, placeholder: "-5.4294")
5. Longitude (number, optional, placeholder: "105.2610")
6. Wilayah/Kelurahan (input text, optional)
7. Foto (file input, optional, JPG/PNG ≤ 5 MB, preview thumbnail)

**Perilaku**:
- Submit → `POST /api/v1/reports` (multipart)
- Sukses: tampilkan modal sukses dengan nomor laporan, tombol "Lihat Riwayat"
- Gagal: tampilkan error per field di bawah input

**States**: Loading kategori (skeleton select), Error foto (pesan di bawah file input), Loading submit (spinner tombol), Sukses (modal)

---

### P-06 – Riwayat Laporan (Warga)
**URL**: `/warga/riwayat.html` (guard: wajib login, role masyarakat)
**Isi**:
- Filter status (All | Menunggu | Diproses | Selesai) – tab atau select
- Daftar laporan sebagai kartu (mobile) / tabel (desktop)
- Tiap kartu/baris: nomor, kategori, lokasi (dipotong), status badge, tanggal, tombol Detail
- Pagination (prev/next + halaman)

**States**: Loading (skeleton kartu), Kosong (empty state), Error (banner), Sukses (daftar)

---

### P-07 – Detail Laporan (Warga)
**URL**: `/warga/detail-laporan.html?id={uuid}` (guard: wajib login)
**Isi**:
- Nomor laporan, kategori, status badge terkini
- Deskripsi lengkap
- Lokasi (alamat + koordinat jika ada)
- Foto (jika ada, klik untuk memperbesar)
- Lini masa riwayat status (timeline vertikal): tanggal, status badge, keterangan, nama admin
- Tombol Kembali

**States**: Loading (skeleton), Error 404 (pesan + tombol kembali), Sukses (detail)

---

### P-08 – Dashboard Admin
**URL**: `/admin/dashboard.html` (guard: wajib login, role admin)
**Isi**:
- 4 kartu statistik: Total Laporan | Menunggu | Diproses | Selesai
- Kartu: "Laporan Hari Ini" (jumlah)
- Tabel "10 Laporan Terbaru" (nomor, pelapor, kategori, status badge, tanggal)
- Grafik donat Chart.js: laporan per kategori (dengan label jumlah)

**States**: Loading (skeleton kartu), Kosong AC-15 (nilai 0 + "Belum ada laporan"), Error (banner), Sukses (data)

---

### P-09 – Daftar Laporan Admin
**URL**: `/admin/laporan.html` (guard: wajib login, role admin)
**Isi**:
- Filter bar: status (select) | kategori (select) | tanggal dari-sampai (date input) | kata kunci (text)
- Tombol "Filter" dan "Reset"
- Tabel laporan: nomor, pelapor, kategori, lokasi, status badge, tanggal, tombol Detail
- Di mobile: kartu list
- Pagination

**States**: Loading (skeleton), Kosong (empty state), Error (banner), Sukses (tabel/list)

---

### P-10 – Detail Laporan Admin
**URL**: `/admin/detail-laporan.html?id={uuid}` (guard: wajib login, role admin)
**Isi**:
- Semua informasi laporan (sama seperti P-07)
- Foto (klik besar)
- Lini masa riwayat
- **Panel Ubah Status** (hanya jika status bukan `selesai`):
  - Select status tujuan (hanya opsi valid sesuai transisi)
  - Textarea keterangan (required)
  - Tombol "Ubah Status" → modal konfirmasi
- Modal konfirmasi: tampilkan status baru + keterangan, tombol Simpan/Batal

**Perilaku**:
- Submit ubah status → `PATCH /api/v1/admin/reports/{id}/status`
- Sukses: refresh halaman + toast "Status berhasil diubah"
- Gagal 422: tampilkan pesan error

---

### P-11 – Profil Admin
**URL**: `/admin/profil.html` (guard: wajib login, role admin)
**Isi**:
- Nama, email, role "admin", tanggal bergabung
- Tombol "Keluar"

---

## 4. Keputusan Konflik PRD vs Desain

| # | Konflik | Keputusan | Alasan |
|---|---|---|---|
| 1 | Desain memakai Tailwind, PRD menetapkan Bootstrap 5 | **Bootstrap 5** | PRD lebih tinggi untuk tech stack |
| 2 | Desain memiliki kartu IoT/sensor (pH, NTU, tekanan) | **Tidak diimplementasi** | SKILL.md § Non-goals |
| 3 | Desain memiliki "Peta Titik Krisis" | **Tidak diimplementasi** | Fase 2 |
| 4 | Desain memiliki badge "Prioritas" | **Tidak diimplementasi** | Tidak ada di PRD |
| 5 | Desain memiliki status "Kendala" | **Tidak diimplementasi** | PRD hanya 3 status |
| 6 | Desain memiliki "Hubungi Petugas" | **Tidak diimplementasi** | Non-goal (chat) |
| 7 | Desain memiliki NIK & nomor telepon | **Tidak diimplementasi** | Tidak ada di PRD |
| 8 | Desain memiliki tombol "Bagikan" | **Tidak diimplementasi** | Non-goal |
| 9 | Logo di desain memakai URL googleusercontent | **Placeholder PNG lokal** | SKILL.md: jangan hotlink |
| 10 | Label "Dosen Pembimbing" di footer | **Tidak ditambahkan** | Pertanyaan terbuka PRD #2, default tidak ada label |
| 11 | File desain Figma tidak tersedia | **Diturunkan dari PRD bagian 13** | Dicatat sebagai asumsi |

---

## 5. Navigasi & Guard JavaScript

```javascript
// static/js/guard.js
// Redirect jika belum login atau role salah

function requireAuth(role) {
  const token = localStorage.getItem('ww_token');
  if (!token) {
    window.location.href = '/login.html';
    return false;
  }
  if (role) {
    const payload = JSON.parse(atob(token.split('.')[1]));
    if (payload.role !== role) {
      window.location.href = payload.role === 'admin'
        ? '/admin/dashboard.html'
        : '/warga/riwayat.html';
      return false;
    }
  }
  return true;
}
```

---

## 6. Struktur File Frontend

```
static/
├── index.html
├── login.html
├── register.html
├── css/
│   └── app.css              (import tokens.css + override Bootstrap)
├── js/
│   ├── api.js               (fetch wrapper + token header)
│   ├── layout.js            (muat footer, inject shell navigasi)
│   ├── guard.js             (auth guard)
│   └── utils.js             (escapeHtml, formatDate, formatStatus)
├── partials/
│   └── footer.html
├── img/
│   ├── logo-pemda.png       (placeholder – TODO: ganti logo resmi)
│   └── logo-uti.png         (placeholder – TODO: ganti logo resmi)
├── warga/
│   ├── profil.html
│   ├── buat-laporan.html
│   ├── riwayat.html
│   └── detail-laporan.html
└── admin/
    ├── dashboard.html
    ├── laporan.html
    ├── detail-laporan.html
    └── profil.html
```

