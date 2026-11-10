// static/js/api.js – Fetch wrapper dengan token header

const API_BASE = '/api/v1';

function getToken() { return localStorage.getItem('ww_token'); }
function setToken(t) { localStorage.setItem('ww_token', t); }
function getUser()  { try { return JSON.parse(localStorage.getItem('ww_user') || 'null'); } catch { return null; } }
function setUser(u) { localStorage.setItem('ww_user', JSON.stringify(u)); }

function clearSession() {
  localStorage.removeItem('ww_token');
  localStorage.removeItem('ww_user');
}

async function apiFetch(path, options = {}) {
  const token = getToken();
  const headers = { ...(options.headers || {}) };

  if (!(options.body instanceof FormData)) {
    headers['Content-Type'] = 'application/json';
  }
  if (token) headers['Authorization'] = `Bearer ${token}`;

  const res = await fetch(`${API_BASE}${path}`, { ...options, headers });

  if (res.status === 401) {
    clearSession();
    window.location.href = '/login.html';
    return;
  }
  if (res.status === 204) return null;

  const data = await res.json().catch(() => null);
  if (!res.ok) throw { status: res.status, ...(data || { error: { message: 'Kesalahan tidak diketahui' } }) };
  return data;
}

window.api = {
  get:    (path, opts)       => apiFetch(path, { ...opts, method: 'GET' }),
  post:   (path, body, opts) => apiFetch(path, { ...opts, method: 'POST',  body: body instanceof FormData ? body : JSON.stringify(body) }),
  patch:  (path, body, opts) => apiFetch(path, { ...opts, method: 'PATCH', body: JSON.stringify(body) }),
  getToken, setToken, getUser, setUser, clearSession,
};

