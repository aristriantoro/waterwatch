# Testing Checklist – WaterWatch v1.0

> Checklist AC-01..AC-24 untuk Fase 4 Pengujian

---

## 1. Checklist Acceptance Criteria

### F-01 – Registrasi & Login

| ID | Given | When | Then | Status |
|---|---|---|---|---|
| **AC-01** | Email belum terdaftar, data valid | Register dikirim | Akun dibuat, role `masyarakat`, password sebagai hash Argon2id | ☑ |
| **AC-02** | Email sudah terdaftar | Register dikirim | HTTP 409 | ☑ |
| **AC-03** | Kredensial benar | Login | JWT dikembalikan, redirect sesuai role | ☑ |
| **AC-04** | Kredensial salah | Login | HTTP 401, pesan generik (tidak bocorkan email mana yang ada) | ☑ |
| **AC-05** | Role masyarakat | Akses `/admin/*` | HTTP 403 | ☑ |

### F-02..F-04 – Pelaporan

| ID | Given | When | Then | Status |
|---|---|---|---|---|
| **AC-06** | Warga login, data valid (kategori, deskripsi 10+ kar, lokasi) | Laporan dikirim | HTTP 201, status `menunggu`, nomor `WW-YYYYMMDD-NNNN` unik, 1 baris `report_updates` | ☑ |
| **AC-07** | Deskripsi < 10 karakter atau lokasi kosong | Laporan dikirim | HTTP 422, pesan error per field | ☑ |
| **AC-08** | Foto JPG/PNG ≤ 5 MB | Foto diunggah | File tersimpan nama UUID, tampil di detail laporan | ☑ |
| **AC-09** | File bukan gambar (PDF, exe) atau > 5 MB | Foto diunggah | HTTP 415 atau 413, laporan TIDAK tersimpan | ☑ |
| **AC-10** | Latitude di luar -90..90 atau longitude di luar -180..180 | Dikirim | HTTP 422 | ☑ |

### F-05 & F-08 – Status & Riwayat

| ID | Given | When | Then | Status |
|---|---|---|---|---|
| **AC-11** | Warga punya beberapa laporan | Buka riwayat | Hanya laporan miliknya, terurut terbaru, pagination 10/hal | ☑ |
| **AC-12** | Warga coba buka laporan milik orang lain | Request dikirim | HTTP 403 atau 404 (tidak bocorkan keberadaan) | ☑ |
| **AC-13** | Admin ubah status laporan | Warga buka detail | Status terbaru + lini masa perubahan tampil | ☑ |

### F-06 – Dashboard Admin

| ID | Given | When | Then | Status |
|---|---|---|---|---|
| **AC-14** | Terdapat data laporan | Admin buka dashboard | Total, per status, hari ini, 10 terbaru akurat terhadap DB | ☑ |
| **AC-15** | Belum ada laporan | Dashboard dibuka | Nilai 0, pesan "Belum ada laporan", tanpa error | ☑ |

### F-07 – Manajemen Laporan

| ID | Given | When | Then | Status |
|---|---|---|---|---|
| **AC-16** | Admin login | Filter status/kategori/tanggal | Hanya data sesuai filter tampil | ☑ |
| **AC-17** | Laporan status `menunggu` | Admin ubah ke `diproses` + keterangan | Status berubah, `report_updates` bertambah dengan `updated_by` = ID admin | ☑ |
| **AC-18** | Laporan status `selesai` | Admin coba ubah status | HTTP 422 (final, tidak bisa diubah) | ☑ |
| **AC-19** | Perubahan status | Dikirim tanpa keterangan | HTTP 422 | ☑ |

### Footer & Lintas Fitur

| ID | Given | When | Then | Status |
|---|---|---|---|---|
| **AC-20** | Pengguna buka halaman apa pun | — | Footer tampil: logo Pemda, logo UTI, "Copyright © 2026 by Aris Triantoro", "Dr. Erlyan Redy Susanto, M.Kom" | ☐ |
| **AC-21** | Layar ≤ 576px | — | Footer terbaca, logo tidak terpotong atau saling menimpa | ☐ |
| **AC-22** | File logo gagal dimuat | — | Teks alt tampil, footer tidak rusak (layout tidak pecah) | ☐ |
| **AC-23** | 50 pengguna bersamaan | Gunakan k6/wrk/ab | P95 respons API ≤ 500 ms | ☐ |
| **AC-24** | Input mengandung `<script>` atau SQL injection (`'; DROP TABLE`) | Form dikirim | Input di-escape/ditolak, tidak dieksekusi | ☐ |

---

## 2. Uji Tambahan (di luar AC tetapi wajib)

### Keamanan
- [ ] `GET /api/v1/admin/dashboard` tanpa token → 401
- [ ] `GET /api/v1/admin/dashboard` dengan token warga → 403
- [ ] `GET /api/v1/reports/{id}` milik orang lain dengan token warga → 404
- [ ] Upload file dengan ekstensi `.png` tapi konten tidak valid (magic bytes bukan PNG) → 415
- [ ] Login 10 kali berturut-turut (password salah) → HTTP 429 setelah batas

### UI / 4 States
- [ ] Halaman riwayat saat loading → skeleton tampil
- [ ] Halaman riwayat saat kosong → empty state "Belum ada laporan"
- [ ] Halaman riwayat saat error API → banner error merah
- [ ] Form buat laporan saat submit sukses → modal sukses dengan nomor laporan
- [ ] Semua halaman di mobile 390px lebar → tidak ada overflow horizontal
- [ ] Semua halaman di desktop 1440px lebar → layout sidebar tidak rusak

### Transisi Status
- [ ] `menunggu` → `diproses` → valid ✓
- [ ] `diproses` → `selesai` → valid ✓
- [ ] `menunggu` → `selesai` (lompat) → 422 ✗
- [ ] `diproses` → `menunggu` (mundur) → 422 ✗
- [ ] `selesai` → apa pun → 422 ✗

---

## 3. Data Simulasi (Fase 4)

Seed data minimum untuk pengujian:

| Data | Jumlah minimum |
|---|---|
| Admin | 2 akun (`admin@waterwatch.id`, `admin2@waterwatch.id`) |
| Warga | ≥ 10 akun (berbeda email) |
| Laporan total | ≥ 50 laporan |
| Laporan per status | Distribusi: ~20 menunggu, ~15 diproses, ~15 selesai |
| Laporan per kategori | Semua 4 kategori terisi |
| Laporan dengan foto | ≥ 10 laporan |
| Laporan dengan koordinat | ≥ 10 laporan |
| Laporan dengan riwayat ≥2 update | ≥ 10 laporan |

---

## 4. Uji Beban (AC-23)

**Tool**: `k6` atau `wrk` atau Apache Benchmark (`ab`)

**Skenario**:
```
50 virtual users bersamaan
Durasi: 1 menit
Endpoints yang diuji:
  - GET /health (baseline)
  - GET /api/v1/categories
  - GET /api/v1/reports/mine (dengan token warga)
  - GET /api/v1/admin/dashboard (dengan token admin)
  - GET /api/v1/admin/reports (dengan token admin)

Target: P95 ≤ 500 ms
```

**Script k6 dasar**:
```javascript
import http from 'k6/http';
import { check, sleep } from 'k6';

const TOKEN_WARGA = __ENV.TOKEN_WARGA;
const TOKEN_ADMIN = __ENV.TOKEN_ADMIN;

export const options = {
  vus: 50,
  duration: '60s',
  thresholds: {
    http_req_duration: ['p(95)<500'],
  },
};

export default function () {
  const headers = { Authorization: `Bearer ${TOKEN_WARGA}` };
  const adminHeaders = { Authorization: `Bearer ${TOKEN_ADMIN}` };

  http.get('http://localhost:8080/health');
  http.get('http://localhost:8080/api/v1/categories');
  http.get('http://localhost:8080/api/v1/reports/mine', { headers });
  http.get('http://localhost:8080/api/v1/admin/dashboard', { headers: adminHeaders });

  sleep(1);
}
```

---

## 5. Uji Tampilan Responsif

| Lebar Layar | Device Target | Yang Dicek |
|---|---|---|
| 390px | iPhone 14 / Android modern | Bottom nav tampil, tabel → kartu, form tidak overflow, footer logo vertikal |
| 768px | iPad / tablet | Transisi sidebar vs bottom nav |
| 1440px | Desktop | Sidebar 256px, layout dua kolom, tabel penuh |

---

## 6. Urutan Eksekusi Fase 4

1. `cargo run --bin seed` → seed admin + data simulasi (≥50 laporan, ≥10 warga)
2. `docker compose up` → jalankan stack
3. Jalankan tes integrasi: `cargo test -- --test-threads=1`
4. Verifikasi AC-01..AC-24 manual atau via script
5. Jalankan uji beban k6
6. Uji tampilan di Chrome DevTools (390px & 1440px)
7. Checklist logo rusak: matikan server statis, buka halaman, cek footer
8. Catat hasil di `docs/laporan-pengujian.md`

---

## 7. Definition of Done

Tiap fitur dinyatakan **DONE** jika:
- [ ] AC terkait lulus (centang di atas)
- [ ] Halaman tampil benar di 390px dan 1440px
- [ ] 4 state UI tertangani (loading, kosong, error, sukses)
- [ ] Footer tampil di halaman tersebut
- [ ] Tidak ada `innerHTML` dengan data pengguna
- [ ] Tidak ada SQL string-concat
- [ ] `docs/CHANGELOG.md` diperbarui
- [ ] `docs/DECISIONS.md` diperbarui (jika ada keputusan baru)

