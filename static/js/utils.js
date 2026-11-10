// static/js/utils.js – Helper render aman (textContent / escapeHtml)

function escapeHtml(str) {
  if (str == null) return '';
  return String(str)
    .replace(/&/g, '&amp;').replace(/</g, '&lt;')
    .replace(/>/g, '&gt;').replace(/"/g, '&quot;').replace(/'/g, '&#x27;');
}

function formatDate(iso) {
  if (!iso) return '-';
  return new Date(iso).toLocaleString('id-ID', {
    year: 'numeric', month: 'long', day: 'numeric',
    hour: '2-digit', minute: '2-digit'
  });
}

function formatDateShort(iso) {
  if (!iso) return '-';
  return new Date(iso).toLocaleDateString('id-ID', {
    year: 'numeric', month: 'short', day: 'numeric'
  });
}

/** Alias formatDate dalam Bahasa Indonesia */
function formatTanggal(iso) { return formatDate(iso); }

const STATUS_CFG = {
  menunggu: { icon: 'schedule',     css: 'ww-badge-menunggu', label: 'Menunggu' },
  diproses: { icon: 'sync',         css: 'ww-badge-diproses', label: 'Diproses' },
  selesai:  { icon: 'check_circle', css: 'ww-badge-selesai',  label: 'Selesai'  },
};

// Kembalikan string HTML aman untuk badge status (label = konstanta sistem, bukan data pengguna)
function statusBadgeHtml(status) {
  const c = STATUS_CFG[status] || { icon: 'help', css: '', label: 'Tidak diketahui' };
  return `<span class="ww-badge ${c.css}"><span class="material-symbols-outlined" aria-hidden="true">${c.icon}</span>${c.label}</span>`;
}

function showToast(message, type = 'success') {
  const el = document.getElementById('app-toast');
  if (!el) return;
  el.className = `toast align-items-center text-bg-${type} border-0 position-fixed bottom-0 end-0 m-3`;
  const msgEl = document.getElementById('toast-message');
  if (msgEl) msgEl.textContent = message; // textContent!
  new bootstrap.Toast(el, { delay: 3500 }).show();
}

function showBanner(containerId, message, type = 'danger') {
  const el = document.getElementById(containerId);
  if (!el) return;
  el.innerHTML = `<div class="alert alert-${type} d-flex align-items-center gap-2 mt-3" role="alert">
    <span class="material-symbols-outlined">error</span>
    <span>${escapeHtml(message)}</span></div>`;
}

function clearBanner(containerId) {
  const el = document.getElementById(containerId);
  if (el) el.innerHTML = '';
}

function skeleton(lines = 3) {
  return Array.from({length: lines}, () =>
    `<div class="placeholder-glow mb-2"><div class="placeholder col-${6+Math.floor(Math.random()*4)} rounded" style="height:18px"></div></div>`
  ).join('');
}

/** Set textContent elemen secara aman (mencegah XSS) */
function setTextSafe(id, value) {
  const el = document.getElementById(id);
  if (el) el.textContent = (value != null) ? String(value) : '-';
}

window.utils = {
  escapeHtml, formatDate, formatDateShort, formatTanggal,
  statusBadgeHtml, showToast, showBanner, clearBanner, skeleton, setTextSafe
};
