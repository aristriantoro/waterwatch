# Frontend Guide – WaterWatch v1.0

> Stack: HTML + CSS + JS vanilla + Bootstrap 5 + Chart.js
> Lihat `references/04-design-system.md` untuk token warna & komponen
> Lihat `references/05-ui-pages.md` untuk daftar halaman & layout

---

## 1. Struktur File Frontend

```
static/
├── index.html
├── login.html
├── register.html
├── css/
│   ├── tokens.css           (dari assets/tokens.css)
│   └── app.css              (import tokens + override Bootstrap)
├── js/
│   ├── api.js               (fetch wrapper + token header)
│   ├── layout.js            (muat footer, inject shell navigasi)
│   ├── guard.js             (auth guard)
│   └── utils.js             (escapeHtml, formatDate, formatStatus, renderBadge)
├── partials/
│   └── footer.html
├── img/
│   ├── logo-pemda.png       (placeholder – TODO: ganti logo resmi Pemda Lampung)
│   └── logo-uti.png         (placeholder – TODO: ganti logo resmi UTI)
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

---

## 2. api.js – Fetch Wrapper

```javascript
// static/js/api.js

const API_BASE = '/api/v1';

function getToken() {
  return localStorage.getItem('ww_token');
}

function setToken(token) {
  localStorage.setItem('ww_token', token);
}

function clearToken() {
  localStorage.removeItem('ww_token');
  localStorage.removeItem('ww_user');
}

async function apiFetch(path, options = {}) {
  const token = getToken();
  const headers = {
    'Content-Type': 'application/json',
    ...(token ? { 'Authorization': `Bearer ${token}` } : {}),
    ...(options.headers || {}),
  };

  // Jika FormData, hapus Content-Type (browser akan set multipart)
  if (options.body instanceof FormData) {
    delete headers['Content-Type'];
  }

  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers,
  });

  if (res.status === 401) {
    clearToken();
    window.location.href = '/login.html';
    return;
  }

  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: { message: 'Kesalahan tidak diketahui' } }));
    throw { status: res.status, ...err };
  }

  // 204 No Content
  if (res.status === 204) return null;
  return res.json();
}

// Export-like pattern untuk vanilla JS
window.api = {
  get:   (path, opts) => apiFetch(path, { ...opts, method: 'GET' }),
  post:  (path, body, opts) => apiFetch(path, { ...opts, method: 'POST', body: body instanceof FormData ? body : JSON.stringify(body) }),
  patch: (path, body, opts) => apiFetch(path, { ...opts, method: 'PATCH', body: JSON.stringify(body) }),
  getToken, setToken, clearToken,
};
```

---

## 3. utils.js – Helper Render Aman

```javascript
// static/js/utils.js

// WAJIB: gunakan ini untuk semua teks dari pengguna – JANGAN innerHTML langsung
function escapeHtml(str) {
  if (str == null) return '';
  return String(str)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#x27;');
}

function formatDate(isoString) {
  if (!isoString) return '-';
  return new Date(isoString).toLocaleString('id-ID', {
    year: 'numeric', month: 'long', day: 'numeric',
    hour: '2-digit', minute: '2-digit'
  });
}

function formatDateShort(isoString) {
  if (!isoString) return '-';
  return new Date(isoString).toLocaleDateString('id-ID', {
    year: 'numeric', month: 'short', day: 'numeric'
  });
}

// Status badge: selalu ikon + warna + teks
const STATUS_CONFIG = {
  menunggu: { icon: 'schedule',     cssClass: 'ww-badge-menunggu', label: 'Menunggu' },
  diproses: { icon: 'sync',         cssClass: 'ww-badge-diproses', label: 'Diproses' },
  selesai:  { icon: 'check_circle', cssClass: 'ww-badge-selesai',  label: 'Selesai'  },
};

function renderStatusBadge(status) {
  const cfg = STATUS_CONFIG[status] || { icon: 'help', cssClass: '', label: escapeHtml(status) };
  // Gunakan textContent equivalent via DOM building – TIDAK innerHTML dengan data pengguna
  const span = document.createElement('span');
  span.className = `ww-badge ${cfg.cssClass}`;

  const icon = document.createElement('span');
  icon.className = 'material-symbols-outlined';
  icon.setAttribute('aria-hidden', 'true');
  icon.textContent = cfg.icon;  // textContent aman

  const text = document.createTextNode(cfg.label);  // aman

  span.appendChild(icon);
  span.appendChild(text);
  return span;
}

// Shorthand: kembalikan string HTML aman untuk badge (tanpa data pengguna di dalamnya)
function statusBadgeHtml(status) {
  const cfg = STATUS_CONFIG[status] || { icon: 'help', cssClass: '', label: 'Tidak diketahui' };
  // Label status adalah konstanta sistem, bukan data pengguna → aman
  return `<span class="ww-badge ${cfg.cssClass}">
    <span class="material-symbols-outlined" aria-hidden="true">${cfg.icon}</span>
    ${cfg.label}
  </span>`;
}

window.utils = { escapeHtml, formatDate, formatDateShort, renderStatusBadge, statusBadgeHtml };
```

---

## 4. guard.js – Auth Guard

```javascript
// static/js/guard.js

function getPayload() {
  const token = localStorage.getItem('ww_token');
  if (!token) return null;
  try {
    return JSON.parse(atob(token.split('.')[1]));
  } catch {
    return null;
  }
}

// Panggil di awal setiap halaman terproteksi
// role: 'masyarakat' | 'admin' | null (hanya cek login)
function requireAuth(requiredRole) {
  const payload = getPayload();
  if (!payload || Date.now() / 1000 > payload.exp) {
    localStorage.removeItem('ww_token');
    window.location.href = '/login.html';
    return null;
  }
  if (requiredRole && payload.role !== requiredRole) {
    // Redirect ke halaman yang sesuai role
    window.location.href = payload.role === 'admin'
      ? '/admin/dashboard.html'
      : '/warga/riwayat.html';
    return null;
  }
  return payload;
}

// Redirect jika sudah login (untuk halaman login/register)
function redirectIfLoggedIn() {
  const payload = getPayload();
  if (payload && Date.now() / 1000 < payload.exp) {
    window.location.href = payload.role === 'admin'
      ? '/admin/dashboard.html'
      : '/warga/riwayat.html';
  }
}

window.guard = { requireAuth, redirectIfLoggedIn, getPayload };
```

---

## 5. layout.js – Footer & Shell Navigasi

```javascript
// static/js/layout.js
// Muat footer di semua halaman

async function loadFooter() {
  const container = document.getElementById('ww-footer-container');
  if (!container) return;
  try {
    const res = await fetch('/static/partials/footer.html');
    if (res.ok) {
      container.innerHTML = await res.text();  // footer.html adalah konten statis (bukan data pengguna)
    }
  } catch (e) {
    // Footer gagal dimuat – tampilkan fallback minimal
    container.innerHTML = `
      <footer class="ww-footer text-center py-3 border-top">
        <p class="mb-0">Copyright &copy; 2026 by Aris Triantoro</p>
        <p class="mb-0">Dr. Erlyan Redy Susanto, M.Kom</p>
      </footer>`;
  }
}

document.addEventListener('DOMContentLoaded', loadFooter);
```

> **Catatan**: `footer.html` adalah file statis yang dikelola developer, bukan data pengguna.
> `innerHTML` pada konten statis ini diizinkan. Data pengguna TETAP harus memakai `textContent` / `escapeHtml`.

---

## 6. Pola Render Aman (Aturan DOM)

```javascript
// ✅ BENAR – gunakan textContent untuk data pengguna
element.textContent = userData.nama;

// ✅ BENAR – gunakan escapeHtml saat membangun string HTML
container.innerHTML = `<p>${utils.escapeHtml(userData.deskripsi)}</p>`;

// ✅ BENAR – gunakan DOM API untuk elemen kompleks
const p = document.createElement('p');
p.textContent = userData.lokasi;
container.appendChild(p);

// ❌ SALAH – JANGAN
container.innerHTML = `<p>${userData.deskripsi}</p>`;  // XSS!
element.innerHTML = userData.nama;                       // XSS!
```

---

## 7. Pola 4-State UI (wajib untuk setiap halaman)

Setiap halaman yang memuat data dari API harus menangani 4 state:

```javascript
async function loadData() {
  // 1. LOADING – tampilkan skeleton
  showSkeleton();

  try {
    const data = await api.get('/some/endpoint');

    if (data.length === 0) {
      // 2. KOSONG – tampilkan empty state
      showEmptyState();
    } else {
      // 3. SUKSES – render data
      renderData(data);
    }
  } catch (err) {
    // 4. ERROR – tampilkan banner error
    showError(err?.error?.message || 'Gagal memuat data. Coba lagi.');
  }
}

function showSkeleton() {
  document.getElementById('content').innerHTML = `
    <div class="placeholder-glow">
      <div class="placeholder col-6 mb-2 rounded" style="height:20px"></div>
      <div class="placeholder col-4 mb-2 rounded" style="height:20px"></div>
      <div class="placeholder col-8 rounded" style="height:20px"></div>
    </div>`;
}

function showEmptyState() {
  document.getElementById('content').innerHTML = `
    <div class="text-center py-5 text-muted">
      <span class="material-symbols-outlined" style="font-size:3rem">inbox</span>
      <p class="mt-2 mb-0">Belum ada data</p>
    </div>`;
}

function showError(message) {
  document.getElementById('content').innerHTML = `
    <div class="alert alert-danger d-flex align-items-center gap-2" role="alert">
      <span class="material-symbols-outlined">error</span>
      <span>${utils.escapeHtml(message)}</span>
    </div>`;
}

function showToast(message, type = 'success') {
  // Gunakan Bootstrap Toast API
  const toastEl = document.getElementById('app-toast');
  document.getElementById('toast-message').textContent = message;  // textContent!
  new bootstrap.Toast(toastEl).show();
}
```

---

## 8. Pola Halaman (Template Dasar)

```html
<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>WaterWatch – [Nama Halaman]</title>
  <!-- Bootstrap 5 -->
  <link rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" />
  <!-- Material Symbols -->
  <link rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />
  <!-- Plus Jakarta Sans -->
  <link rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700&display=swap" />
  <!-- WaterWatch styles -->
  <link rel="stylesheet" href="/static/css/tokens.css" />
  <link rel="stylesheet" href="/static/css/app.css" />
</head>
<body class="ww-bg">

  <!-- [Shell navigasi (sidebar/bottom-nav) diinjeksi oleh layout.js atau ditulis langsung] -->

  <main id="main-content" class="p-3">
    <!-- Konten halaman -->
    <div id="content">
      <!-- Loading / data / empty / error -->
    </div>
  </main>

  <!-- Footer container – diisi oleh layout.js -->
  <div id="ww-footer-container"></div>

  <!-- Toast global -->
  <div class="toast align-items-center text-bg-success border-0 position-fixed bottom-0 end-0 m-3"
       role="alert" aria-live="assertive" id="app-toast" style="z-index:9999">
    <div class="d-flex">
      <div class="toast-body">
        <span class="material-symbols-outlined me-1 align-middle">check_circle</span>
        <span id="toast-message"></span>
      </div>
      <button type="button" class="btn-close btn-close-white me-2 m-auto"
              data-bs-dismiss="toast" aria-label="Tutup"></button>
    </div>
  </div>

  <!-- Scripts -->
  <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
  <script src="/static/js/utils.js"></script>
  <script src="/static/js/api.js"></script>
  <script src="/static/js/guard.js"></script>
  <script src="/static/js/layout.js"></script>
  <script src="/static/js/[halaman].js"></script>
</body>
</html>
```

---

## 9. Navigasi Bottom (Mobile Warga)

```html
<nav class="ww-bottom-nav navbar fixed-bottom border-top bg-white d-md-none" aria-label="Navigasi utama">
  <div class="container-fluid justify-content-around">
    <a href="/warga/riwayat.html" class="ww-nav-item text-center text-decoration-none">
      <span class="material-symbols-outlined d-block">history</span>
      <small>Riwayat</small>
    </a>
    <a href="/warga/buat-laporan.html" class="ww-nav-item text-center text-decoration-none">
      <span class="material-symbols-outlined d-block">add_circle</span>
      <small>Laporan</small>
    </a>
    <a href="/warga/profil.html" class="ww-nav-item text-center text-decoration-none">
      <span class="material-symbols-outlined d-block">person</span>
      <small>Profil</small>
    </a>
  </div>
</nav>
```

```css
/* app.css */
.ww-bottom-nav {
  height: var(--ww-bottom-nav-height);
  padding-bottom: env(safe-area-inset-bottom);
}
.ww-nav-item {
  min-width: 44px;
  min-height: 44px;    /* Target sentuh ≥ 44px */
  padding: 0.25rem 0.75rem;
  color: var(--ww-text-secondary);
}
.ww-nav-item.active,
.ww-nav-item:hover {
  color: var(--ww-primary);
}
```

---

## 10. Checklist Keamanan Frontend

- [ ] Semua teks dari API menggunakan `textContent` atau `utils.escapeHtml()`
- [ ] Tidak ada `element.innerHTML = userDerivedData` tanpa escaping
- [ ] Token disimpan di `localStorage` (acceptable untuk MVP SPA)
- [ ] Token dihapus saat logout (`clearToken()`)
- [ ] Guard dipanggil di awal setiap halaman terproteksi
- [ ] Form validation dilakukan di sisi client (UX) DAN server (keamanan)
- [ ] Foto preview tidak memakai URL blob dari nama file asli
- [ ] Error dari API ditampilkan via `escapeHtml(err.error.message)`

