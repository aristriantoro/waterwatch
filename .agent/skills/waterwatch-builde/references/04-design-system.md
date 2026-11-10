# Design System – WaterWatch v1.0

> **Catatan**: File desain UI/UX (Figma) tidak tersedia.
> Design system ini diturunkan dari PRD v1.0 bagian 13 dan aturan UI di SKILL.md.
> Lihat `docs/DECISIONS.md` untuk daftar asumsi desain.

---

## 1. Typografi

| Elemen | Font | Ukuran | Weight |
|---|---|---|---|
| Font utama | Plus Jakarta Sans | — | — |
| Heading H1 | Plus Jakarta Sans | 2rem (32px) | 700 |
| Heading H2 | Plus Jakarta Sans | 1.5rem (24px) | 600 |
| Heading H3 | Plus Jakarta Sans | 1.25rem (20px) | 600 |
| Body | Plus Jakarta Sans | 1rem (16px) | 400 |
| Small / caption | Plus Jakarta Sans | 0.875rem (14px) | 400 |
| Label form | Plus Jakarta Sans | 0.875rem (14px) | 500 |

```css
/* Import via Google Fonts CDN */
@import url('https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700&display=swap');

/* Fallback stack */
font-family: 'Plus Jakarta Sans', 'Segoe UI', system-ui, sans-serif;
```

---

## 2. Palet Warna (Token CSS)

```css
/* File: assets/tokens.css */

:root {
  /* ── Brand Primary ───────────────────────── */
  --ww-primary:          #0369A1;   /* Biru air – tombol utama, link aktif */
  --ww-primary-hover:    #0284C7;
  --ww-primary-light:    #E0F2FE;   /* Background chip/badge primary */

  /* ── Brand Secondary ─────────────────────── */
  --ww-secondary:        #0EA5E9;   /* Aksen, ikon */
  --ww-secondary-light:  #BAE6FD;

  /* ── Netral ──────────────────────────────── */
  --ww-bg:               #F8FAFC;   /* Background halaman */
  --ww-surface:          #FFFFFF;   /* Card, modal, sidebar */
  --ww-border:           #E2E8F0;   /* Garis pemisah */
  --ww-text-primary:     #0F172A;   /* Teks utama */
  --ww-text-secondary:   #64748B;   /* Teks sekunder, placeholder */
  --ww-text-disabled:    #CBD5E1;

  /* ── Status ──────────────────────────────── */
  --ww-menunggu-bg:      #FEF9C3;   /* Kuning muda */
  --ww-menunggu-text:    #854D0E;
  --ww-menunggu-border:  #FDE047;

  --ww-diproses-bg:      #DBEAFE;   /* Biru muda */
  --ww-diproses-text:    #1E40AF;
  --ww-diproses-border:  #93C5FD;

  --ww-selesai-bg:       #DCFCE7;   /* Hijau muda */
  --ww-selesai-text:     #166534;
  --ww-selesai-border:   #86EFAC;

  /* ── Feedback ────────────────────────────── */
  --ww-success:          #16A34A;
  --ww-warning:          #D97706;
  --ww-danger:           #DC2626;
  --ww-info:             #0369A1;

  /* ── Layout ──────────────────────────────── */
  --ww-sidebar-width:    256px;
  --ww-topbar-height:    64px;
  --ww-bottom-nav-height:60px;
  --ww-radius-sm:        4px;
  --ww-radius-md:        8px;
  --ww-radius-lg:        12px;

  /* ── Shadow ──────────────────────────────── */
  --ww-shadow-sm:  0 1px 2px 0 rgb(0 0 0 / 0.05);
  --ww-shadow-md:  0 4px 6px -1px rgb(0 0 0 / 0.1);
  --ww-shadow-lg:  0 10px 15px -3px rgb(0 0 0 / 0.1);
}
```

---

## 3. Status Badge Component

Status **selalu** ikon + warna + teks (JANGAN warna saja – aksesibilitas).

```html
<!-- Menunggu -->
<span class="ww-badge ww-badge-menunggu">
  <span class="material-symbols-outlined" aria-hidden="true">schedule</span>
  Menunggu
</span>

<!-- Diproses -->
<span class="ww-badge ww-badge-diproses">
  <span class="material-symbols-outlined" aria-hidden="true">sync</span>
  Diproses
</span>

<!-- Selesai -->
<span class="ww-badge ww-badge-selesai">
  <span class="material-symbols-outlined" aria-hidden="true">check_circle</span>
  Selesai
</span>
```

```css
/* Status badge styles */
.ww-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.75rem;
  border-radius: 999px;
  font-size: 0.8125rem;
  font-weight: 500;
  border: 1px solid;
}
.ww-badge .material-symbols-outlined {
  font-size: 1rem;
}
.ww-badge-menunggu {
  background: var(--ww-menunggu-bg);
  color: var(--ww-menunggu-text);
  border-color: var(--ww-menunggu-border);
}
.ww-badge-diproses {
  background: var(--ww-diproses-bg);
  color: var(--ww-diproses-text);
  border-color: var(--ww-diproses-border);
}
.ww-badge-selesai {
  background: var(--ww-selesai-bg);
  color: var(--ww-selesai-text);
  border-color: var(--ww-selesai-border);
}
```

---

## 4. Layout

### Desktop (≥768px)
- **Sidebar** kiri tetap, lebar `256px`, warna `--ww-surface`, shadow kanan
- **Topbar** kanan atas, tinggi `64px`, berisi nama pengguna + ikon profil
- **Konten utama** sebelah kanan sidebar, padding `1.5rem`
- Tabel laporan ditampilkan sebagai **tabel** dengan kolom

### Mobile (<768px)
- **Bottom navigation** tetap di bawah, tinggi `60px`, berisi 3–4 ikon navigasi utama
- Sidebar tersembunyi; buka via hamburger jika diperlukan
- Tabel laporan ditampilkan sebagai **kartu** (card list), bukan tabel
- **Target sentuh minimum 44×44px** untuk semua elemen interaktif

### Halaman linear (Form Buat Laporan)
- Sembunyikan shell navigasi (sidebar/bottom-nav) untuk fokus
- Tampilkan tombol Batal dan header minimal

---

## 5. Komponen Umum

### Loading State (Skeleton)
```html
<div class="placeholder-glow">
  <div class="placeholder col-6 mb-2 rounded"></div>
  <div class="placeholder col-4 mb-2 rounded"></div>
  <div class="placeholder col-8 rounded"></div>
</div>
```

### Empty State
```html
<div class="text-center py-5 text-muted">
  <span class="material-symbols-outlined" style="font-size:3rem">inbox</span>
  <p class="mt-2 mb-0">Belum ada laporan</p>
</div>
```

### Error Banner
```html
<div class="alert alert-danger d-flex align-items-center gap-2" role="alert">
  <span class="material-symbols-outlined">error</span>
  <span id="error-message">Terjadi kesalahan. Coba lagi.</span>
</div>
```

### Toast (Sukses)
```html
<div class="toast align-items-center text-bg-success border-0 position-fixed bottom-0 end-0 m-3"
     role="alert" aria-live="assertive" id="success-toast">
  <div class="d-flex">
    <div class="toast-body">
      <span class="material-symbols-outlined me-1 align-middle">check_circle</span>
      <span id="toast-message">Berhasil disimpan</span>
    </div>
    <button type="button" class="btn-close btn-close-white me-2 m-auto"
            data-bs-dismiss="toast" aria-label="Tutup"></button>
  </div>
</div>
```

### Modal Konfirmasi (Ubah Status)
```html
<div class="modal fade" id="confirm-status-modal" tabindex="-1">
  <div class="modal-dialog modal-dialog-centered">
    <div class="modal-content">
      <div class="modal-header">
        <h5 class="modal-title">Konfirmasi Perubahan Status</h5>
        <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
      </div>
      <div class="modal-body">
        <p>Ubah status menjadi <strong id="confirm-new-status"></strong>?</p>
        <div class="mb-3">
          <label for="keterangan-input" class="form-label">Keterangan <span class="text-danger">*</span></label>
          <textarea class="form-control" id="keterangan-input" rows="3" required></textarea>
        </div>
      </div>
      <div class="modal-footer">
        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Batal</button>
        <button type="button" class="btn btn-primary" id="confirm-status-btn">Simpan</button>
      </div>
    </div>
  </div>
</div>
```

---

## 6. Ikon

Gunakan **Material Symbols Outlined** via Google Fonts CDN (atau self-host).

```html
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />
```

Mapping ikon per fitur:
| Fitur | Ikon |
|---|---|
| Beranda | `home` |
| Buat Laporan | `add_circle` / `report` |
| Riwayat | `history` |
| Profil | `person` |
| Dashboard | `dashboard` |
| Laporan Masuk | `list_alt` |
| Status menunggu | `schedule` |
| Status diproses | `sync` |
| Status selesai | `check_circle` |
| Foto | `photo_camera` |
| Lokasi | `location_on` |
| Filter | `filter_list` |
| Cari | `search` |
| Logout | `logout` |
| Error | `error` |
| Kosong | `inbox` |

---

## 7. Footer

Lihat `assets/footer.html` dan `references/05-ui-pages.md` bagian Footer.

Aturan:
- Logo: `height: 48px`, dengan `alt` yang bermakna
- Di layar ≤576px: logo ditumpuk vertikal (`flex-direction: column`)
- Jika logo gagal dimuat: `onerror="this.style.display='none'"` agar footer tidak rusak
- Teks: "Copyright © 2026 by Aris Triantoro" dan "Dr. Erlyan Redy Susanto, M.Kom"
- Tanpa label peran ("Dosen Pembimbing") — keputusan terbuka, lihat DECISIONS.md

---

## 8. Prinsip Aksesibilitas

- Semua input form punya `<label>` terikat (`for` + `id`)
- Semua gambar punya `alt`; dekorasi `alt=""`
- Kontras warna WCAG AA minimal
- Navigasi keyboard: Tab order logis
- Status badge selalu teks + ikon (bukan warna saja)
- Pesan error dikaitkan ke field via `aria-describedby`

