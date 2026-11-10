// static/js/guard.js – Auth guard dan redirect

function getPayload() {
  const t = localStorage.getItem('ww_token');
  if (!t) return null;
  try { return JSON.parse(atob(t.split('.')[1])); } catch { return null; }
}

// Panggil di halaman terproteksi. Kembalikan payload atau null (sudah redirect).
function requireAuth(requiredRole) {
  const p = getPayload();
  if (!p || Date.now() / 1000 > p.exp) {
    localStorage.removeItem('ww_token');
    window.location.href = '/login.html';
    return null;
  }
  if (requiredRole && p.role !== requiredRole) {
    window.location.href = p.role === 'admin' ? '/admin/dashboard.html' : '/warga/riwayat.html';
    return null;
  }
  return p;
}

// Shorthand: hanya admin
function requireAdmin() { return requireAuth('admin'); }
// Shorthand: hanya warga (masyarakat)
function requireWarga() { return requireAuth('masyarakat'); }

// Redirect jika sudah login (halaman login/register)
function redirectIfLoggedIn() {
  const p = getPayload();
  if (p && Date.now() / 1000 < p.exp) {
    window.location.href = p.role === 'admin' ? '/admin/dashboard.html' : '/warga/riwayat.html';
  }
}

window.guard = { requireAuth, requireAdmin, requireWarga, redirectIfLoggedIn, getPayload };

