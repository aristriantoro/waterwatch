# DECISIONS.md – WaterWatch

> Log keputusan teknis, asumsi, dan resolusi konflik.
> Format: tanggal | keputusan | alasan | dampak

---

## Asumsi Desain UI/UX

**Konteks**: File desain UI/UX (Figma) tidak tersedia saat pengerjaan dimulai.
**Keputusan**: Design system dan spesifikasi halaman diturunkan langsung dari:
- PRD v1.0 bagian 13 (Gambaran Antarmuka)
- Aturan UI di SKILL.md (Bootstrap 5, Plus Jakarta Sans, Material Symbols, bottom nav mobile, sidebar desktop)
- Contoh kode footer di PRD bagian 10.4

**Dampak**: Tampilan visual bisa berbeda dari desain Figma jika kelak tersedia.
Jika desain Figma tersedia, perbarui `references/04-design-system.md` dan `references/05-ui-pages.md`.

---

## Keputusan Konflik PRD vs Desain

| # | Tanggal | Konflik | Keputusan | Alasan |
|---|---|---|---|---|
| D-01 | 2026-11-10 | Desain memakai Tailwind, PRD menetapkan Bootstrap 5 | **Bootstrap 5** | PRD lebih tinggi untuk tech stack. SKILL.md menegaskan. |
| D-02 | 2026-11-10 | Desain memiliki kartu IoT/sensor (pH, NTU, tekanan) | **Tidak diimplementasi** | SKILL.md § Non-goals; PRD § 3.3 |
| D-03 | 2026-11-10 | Desain memiliki "Peta Titik Krisis" (Leaflet) | **Tidak diimplementasi di MVP** | Fase 2. Tabel `announcements` tetap dibuat. |
| D-04 | 2026-11-10 | Desain memiliki badge "Prioritas" | **Tidak diimplementasi** | Tidak ada di PRD |
| D-05 | 2026-11-10 | Desain memiliki status "Kendala" | **Tidak diimplementasi** | PRD hanya 3 status: menunggu, diproses, selesai |
| D-06 | 2026-11-10 | Desain memiliki "Hubungi Petugas" | **Tidak diimplementasi** | Non-goal (chat) per PRD § 3.3 |
| D-07 | 2026-11-10 | Desain memiliki NIK & nomor telepon | **Tidak diimplementasi** | Tidak ada di PRD. PRD hanya nama + email. |
| D-08 | 2026-11-10 | Desain memiliki tombol "Bagikan" | **Tidak diimplementasi** | Non-goal per SKILL.md |
| D-09 | 2026-11-10 | Logo resmi Pemda & UTI sudah tersedia | **Logo resmi dipakai langsung** | File `logo-pemda.png` & `logo-uti.png` sudah ada di `static/img/`. Tidak dibuat placeholder. |
| D-10 | 2026-11-10 | Label "Dosen Pembimbing" di footer | **Tidak ditambahkan** | Dijawab pemilik produk: hanya nama "Dr. Erlyan Redy Susanto, M.Kom" tanpa label peran. |
| D-11 | 2026-11-10 | Build & runtime Rust | **Semua di Docker (multi-stage)** | Mesin developer belum ada Rust. Builder=`rust:1.80-slim`, Runner=`debian:bookworm-slim`. |

---

## Pertanyaan Terbuka (dari PRD § 17)

| # | Pertanyaan | Status | Keputusan Default |
|---|---|---|---|
| Q-01 | Pemda mana yang menjadi acuan logo dan pengelola layanan? | **✅ Dijawab** | "Pemerintah Provinsi Lampung" |
| Q-02 | Apakah footer perlu label peran ("Dosen Pembimbing") di samping nama? | **✅ Dijawab** | **Tanpa label** – hanya nama "Dr. Erlyan Redy Susanto, M.Kom" |
| Q-03 | Fitur lupa password di MVP? | **✅ Dijawab** | **Ditunda** – ke Fase 2 |
| Q-04 | Daftar wilayah: dropdown baku atau input bebas? | **✅ Dijawab** | **Input teks bebas** (`wilayah VARCHAR(100)`) |

---

## Keputusan Teknis

| # | Tanggal | Topik | Keputusan | Alasan |
|---|---|---|---|---|
| T-01 | 2026-11-10 | Nomor laporan harian | Gunakan tabel `report_number_seq` dengan `ON CONFLICT DO UPDATE counter+1` dalam transaksi | Atomis, sederhana, tidak butuh sequence PG yang kompleks |
| T-02 | 2026-11-10 | Storage foto | Filesystem lokal `/uploads/` (UUID.ext) | PRD menyarankan ini, cukup untuk MVP dan data simulasi |
| T-03 | 2026-11-10 | Validasi tipe foto | `infer` crate (magic bytes) | PRD FR-09 mensyaratkan validasi konten, bukan ekstensi |
| T-04 | 2026-11-10 | Token auth frontend | `localStorage` | Acceptable untuk MVP SPA; catatan: httpOnly cookie lebih aman untuk produksi |
| T-05 | 2026-11-10 | Wilayah | Input teks bebas (VARCHAR 100) | Pertanyaan Q-04 belum dijawab; ini bisa diubah ke dropdown jika ada respons |
| T-06 | 2026-11-10 | Seed admin | `src/bin/seed.rs` binary terpisah | PRD FR-05 menyatakan admin hanya via seed, bukan registrasi publik |
| T-07 | 2026-11-10 | Rate limit login | `tower_governor` 5 req/60 detik per IP | Mencegah brute force (FR-06) |
| T-08 | 2026-11-10 | Pagination default | 10 laporan per halaman | PRD FR-15 |
| T-09 | 2026-11-10 | Endpoint foto | `GET /api/v1/reports/{id}/foto` terotorisasi | Foto hanya pemilik/admin yang bisa akses, sesuai FR-13 |

