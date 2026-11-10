// static/js/api.js – Mock API Service WaterWatch (Simulasi Tanpa Database Eksternal)
// Seluruh data disimpan dan diproses secara interaktif di browser (localStorage)

(function () {
  const DB_KEY = 'ww_mock_database_v1';

  // Seed awal data realistis Provinsi Lampung
  const DEFAULT_DB = {
    users: [
      {
        id: 1,
        nama: 'Administrator PDAM Lampung',
        email: 'admin@waterwatch.id',
        password: 'Admin@1234!',
        role: 'admin',
        created_at: '2026-11-01T08:00:00Z',
      },
      {
        id: 2,
        nama: 'Budi Santoso',
        email: 'warga1@example.com',
        password: 'Warga@1234!',
        role: 'masyarakat',
        created_at: '2026-11-01T08:00:00Z',
      },
      {
        id: 3,
        nama: 'Siti Rahayu',
        email: 'warga2@example.com',
        password: 'Warga@1234!',
        role: 'masyarakat',
        created_at: '2026-11-02T09:00:00Z',
      },
    ],
    categories: [
      { id: 1, nama: 'Pipa Bocor', deskripsi: 'Kebocoran pada pipa distribusi utama atau sambungan rumah' },
      { id: 2, nama: 'Air Keruh / Berbau', deskripsi: 'Kualitas air tidak layak, berwarna coklat/keruh atau berbau menyengat' },
      { id: 3, nama: 'Air Mati / Aliran Kecil', deskripsi: 'Pasokan air terhenti total atau debit sangat kecil di jam sibuk' },
      { id: 4, nama: 'Meteran Rusak', deskripsi: 'Meteran air macet, bocor, atau angka putaran tidak wajar' },
    ],
    reports: [
      {
        id: 1,
        nomor: 'LAP-20261110-0001',
        user_id: 2,
        user_nama: 'Budi Santoso',
        category_id: 1,
        kategori: 'Pipa Bocor',
        deskripsi: 'Pipa PDAM utama di pinggir jalan bocor dan air meluap ke badan jalan aspal sejak pagi. Tekanan air ke rumah warga sekitar menurun drastis.',
        lokasi: 'Jl. ZA. Pagar Alam No. 93, Gedong Meneng, Rajabasa',
        wilayah: 'Kota Bandar Lampung',
        latitude: -5.3812,
        longitude: 105.2578,
        status: 'menunggu',
        foto: 'https://images.unsplash.com/photo-1541888946425-d0fbb186f5f7?auto=format&fit=crop&w=800&q=80',
        created_at: '2026-11-10T08:30:00Z',
        riwayat: [
          {
            status: 'menunggu',
            keterangan: 'Laporan masuk dari masyarakat',
            updated_by_nama: null,
            updated_at: '2026-11-10T08:30:00Z',
          },
        ],
      },
      {
        id: 2,
        nomor: 'LAP-20261109-0002',
        user_id: 2,
        user_nama: 'Budi Santoso',
        category_id: 2,
        kategori: 'Air Keruh / Berbau',
        deskripsi: 'Air keran berwarna kecoklatan dan berbau tanah setelah hujan lebat kemarin sore. Tidak bisa digunakan untuk keperluan konsumsi.',
        lokasi: 'Komplek Way Halim Permai Blok B2, Sukarame',
        wilayah: 'Kota Bandar Lampung',
        latitude: -5.3971,
        longitude: 105.2812,
        status: 'diproses',
        foto: 'https://images.unsplash.com/photo-1584820927498-cfe5211fd8bf?auto=format&fit=crop&w=800&q=80',
        created_at: '2026-11-09T14:15:00Z',
        riwayat: [
          {
            status: 'menunggu',
            keterangan: 'Laporan masuk dari masyarakat',
            updated_by_nama: null,
            updated_at: '2026-11-09T14:15:00Z',
          },
          {
            status: 'diproses',
            keterangan: 'Tim teknis PDAM Wilayah Sukarame sedang melakukan flushing pipa dan pengecekan filter penjernih.',
            updated_by_nama: 'Administrator PDAM Lampung',
            updated_at: '2026-11-09T16:00:00Z',
          },
        ],
      },
      {
        id: 3,
        nomor: 'LAP-20261108-0003',
        user_id: 3,
        user_nama: 'Siti Rahayu',
        category_id: 3,
        kategori: 'Air Mati / Aliran Kecil',
        deskripsi: 'Aliran air PDAM mati total sejak 2 hari berturut-turut tanpa pemberitahuan sebelumnya di seputaran Dusun Hajimena.',
        lokasi: 'Jl. Raden Gunawan, Hajimena, Natar',
        wilayah: 'Kabupaten Lampung Selatan',
        latitude: -5.3421,
        longitude: 105.2314,
        status: 'selesai',
        foto: 'https://images.unsplash.com/photo-1548839140-29a749e1bc4e?auto=format&fit=crop&w=800&q=80',
        created_at: '2026-11-08T10:00:00Z',
        riwayat: [
          {
            status: 'menunggu',
            keterangan: 'Laporan masuk dari masyarakat',
            updated_by_nama: null,
            updated_at: '2026-11-08T10:00:00Z',
          },
          {
            status: 'diproses',
            keterangan: 'Pembersihan intake pompa transmisi Natar yang mengalami pendangkalan.',
            updated_by_nama: 'Administrator PDAM Lampung',
            updated_at: '2026-11-08T13:30:00Z',
          },
          {
            status: 'selesai',
            keterangan: 'Perbaikan selesai dilakukan. Pasokan air bersih sudah kembali mengalir lancar dengan tekanan normal.',
            updated_by_nama: 'Administrator PDAM Lampung',
            updated_at: '2026-11-09T09:00:00Z',
          },
        ],
      },
      {
        id: 4,
        nomor: 'LAP-20261107-0004',
        user_id: 3,
        user_nama: 'Siti Rahayu',
        category_id: 4,
        kategori: 'Meteran Rusak',
        deskripsi: 'Kaca meteran retak dan jarum angka meteran berputar cepat tidak wajar saat seluruh keran air tertutup.',
        lokasi: 'Jl. Jendral Sudirman, Ganjaragung, Metro Barat',
        wilayah: 'Kota Metro',
        latitude: -5.1132,
        longitude: 105.3067,
        status: 'selesai',
        foto: 'https://images.unsplash.com/photo-1581092160607-ee22621dd758?auto=format&fit=crop&w=800&q=80',
        created_at: '2026-11-07T11:20:00Z',
        riwayat: [
          {
            status: 'menunggu',
            keterangan: 'Laporan masuk dari masyarakat',
            updated_by_nama: null,
            updated_at: '2026-11-07T11:20:00Z',
          },
          {
            status: 'diproses',
            keterangan: 'Jadwal kunjungan teknisi untuk penggantian meter air telah diagendakan.',
            updated_by_nama: 'Administrator PDAM Lampung',
            updated_at: '2026-11-07T15:00:00Z',
          },
          {
            status: 'selesai',
            keterangan: 'Meteran air telah diganti dengan unit baru terkalibrasi SNI.',
            updated_by_nama: 'Administrator PDAM Lampung',
            updated_at: '2026-11-08T11:00:00Z',
          },
        ],
      },
    ],
  };

  function getDB() {
    try {
      const raw = localStorage.getItem(DB_KEY);
      if (!raw) {
        localStorage.setItem(DB_KEY, JSON.stringify(DEFAULT_DB));
        return JSON.parse(JSON.stringify(DEFAULT_DB));
      }
      return JSON.parse(raw);
    } catch (e) {
      return JSON.parse(JSON.stringify(DEFAULT_DB));
    }
  }

  function saveDB(db) {
    localStorage.setItem(DB_KEY, JSON.stringify(db));
  }

  function resetSimulationData() {
    localStorage.setItem(DB_KEY, JSON.stringify(DEFAULT_DB));
    console.log('WaterWatch: Data simulasi berhasil direset ke kondisi awal.');
  }

  // Token & Auth Helper
  function getToken() { return localStorage.getItem('ww_token'); }
  function setToken(t) { localStorage.setItem('ww_token', t); }
  function getUser()  { try { return JSON.parse(localStorage.getItem('ww_user') || 'null'); } catch { return null; } }
  function setUser(u) { localStorage.setItem('ww_user', JSON.stringify(u)); }
  function clearSession() {
    localStorage.removeItem('ww_token');
    localStorage.removeItem('ww_user');
  }

  function createSimulatedJwt(user) {
    const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
    const payload = btoa(
      JSON.stringify({
        sub: user.id,
        email: user.email,
        role: user.role,
        nama: user.nama,
        exp: Math.floor(Date.now() / 1000) + 86400 * 30, // 30 hari
      })
    );
    return `${header}.${payload}.simulated_signature`;
  }

  function getCurrentUser() {
    const u = getUser();
    if (u) return u;
    const token = getToken();
    if (!token) return null;
    try {
      const p = JSON.parse(atob(token.split('.')[1]));
      return { id: p.sub, email: p.email, role: p.role, nama: p.nama || 'Pengguna' };
    } catch {
      return null;
    }
  }

  // Helper konversi file gambar ke Base64 Data URL
  async function fileToDataUrl(file) {
    if (!file || !(file instanceof File) || file.size === 0) return null;
    return new Promise((resolve) => {
      const reader = new FileReader();
      reader.onload = (e) => resolve(e.target.result);
      reader.onerror = () => resolve(null);
      reader.readAsDataURL(file);
    });
  }

  // Router Mock API
  async function mockRouter(method, path, body = {}) {
    // Simulasi delay jaringan realistis (100 - 250ms)
    await new Promise((r) => setTimeout(r, 120));

    const db = getDB();
    const cleanPath = path.replace(/^\/api\/v1/, '').split('?')[0];
    const urlParams = new URLSearchParams(path.includes('?') ? path.split('?')[1] : '');

    // 1. POST /auth/login
    if (method === 'POST' && (cleanPath === '/auth/login')) {
      const { email, password } = body || {};
      const user = db.users.find((u) => u.email.toLowerCase() === (email || '').toLowerCase().trim());
      if (!user) {
        throw { status: 401, error: { message: 'Email atau password salah.' } };
      }
      // Demo toleran: cek password atau izinkan password demo
      if (user.password && password && user.password !== password && password !== 'Admin@1234!' && password !== 'Warga@1234!') {
        throw { status: 401, error: { message: 'Password yang Anda masukkan salah.' } };
      }
      const token = createSimulatedJwt(user);
      const userResp = { id: user.id, nama: user.nama, email: user.email, role: user.role, created_at: user.created_at };
      setToken(token);
      setUser(userResp);
      return { token, user: userResp };
    }

    // 2. POST /auth/register
    if (method === 'POST' && (cleanPath === '/auth/register')) {
      const { nama, email, password } = body || {};
      if (!nama || !email || !password) {
        throw { status: 400, error: { message: 'Nama, email, dan password wajib diisi.' } };
      }
      const existing = db.users.find((u) => u.email.toLowerCase() === email.toLowerCase().trim());
      if (existing) {
        throw { status: 409, error: { message: 'Email sudah terdaftar. Silakan gunakan email lain atau login.' } };
      }
      const newUser = {
        id: db.users.length ? Math.max(...db.users.map((u) => u.id)) + 1 : 1,
        nama: nama.trim(),
        email: email.trim().toLowerCase(),
        password: password,
        role: 'masyarakat',
        created_at: new Date().toISOString(),
      };
      db.users.push(newUser);
      saveDB(db);

      const token = createSimulatedJwt(newUser);
      const userResp = { id: newUser.id, nama: newUser.nama, email: newUser.email, role: newUser.role, created_at: newUser.created_at };
      setToken(token);
      setUser(userResp);
      return { token, user: userResp };
    }

    // 3. GET /auth/me
    if (method === 'GET' && (cleanPath === '/auth/me')) {
      const cur = getCurrentUser();
      if (!cur) throw { status: 401, error: { message: 'Silakan masuk terlebih dahulu.' } };
      const userInDb = db.users.find((u) => u.id === cur.id) || cur;
      return { id: userInDb.id, nama: userInDb.nama, email: userInDb.email, role: userInDb.role, created_at: userInDb.created_at };
    }

    // 4. GET /categories
    if (method === 'GET' && (cleanPath === '/categories')) {
      return db.categories;
    }

    // 5. GET /admin/dashboard
    if (method === 'GET' && (cleanPath === '/admin/dashboard')) {
      const total = db.reports.length;
      const menunggu = db.reports.filter((r) => r.status === 'menunggu').length;
      const diproses = db.reports.filter((r) => r.status === 'diproses').length;
      const selesai = db.reports.filter((r) => r.status === 'selesai').length;

      const todayStr = new Date().toISOString().slice(0, 10);
      const hari_ini = db.reports.filter((r) => (r.created_at || '').startsWith(todayStr)).length;

      const per_kategori = db.categories.map((c) => ({
        kategori: c.nama,
        jumlah: db.reports.filter((r) => r.category_id === c.id || r.kategori === c.nama).length,
      }));

      const laporan_terbaru = [...db.reports]
        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
        .slice(0, 5);

      return {
        total,
        per_status: { menunggu, diproses, selesai },
        hari_ini,
        per_kategori,
        laporan_terbaru,
      };
    }

    // 6. GET /reports/mine (Laporan milik warga login)
    if (method === 'GET' && (cleanPath === '/reports/mine')) {
      const cur = getCurrentUser();
      const userId = cur ? cur.id : 2; // default Budi Santoso jika sesi belum tersimpan
      let list = db.reports.filter((r) => r.user_id === userId);

      const statusFilter = urlParams.get('status');
      if (statusFilter) {
        list = list.filter((r) => r.status === statusFilter);
      }

      list.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));

      const page = parseInt(urlParams.get('page') || '1', 10);
      const per_page = parseInt(urlParams.get('per_page') || '10', 10);
      const total = list.length;
      const total_pages = Math.ceil(total / per_page) || 1;
      const start = (page - 1) * per_page;
      const paged = list.slice(start, start + per_page);

      return {
        data: paged,
        meta: { total, page, per_page, total_pages },
      };
    }

    // 7. POST /reports (Buat laporan baru)
    if (method === 'POST' && (cleanPath === '/reports')) {
      const cur = getCurrentUser();
      let kategori_id, deskripsi, lokasi, wilayah, latitude, longitude, fotoFile;

      if (body instanceof FormData) {
        kategori_id = parseInt(body.get('kategori_id') || '1', 10);
        deskripsi = (body.get('deskripsi') || '').trim();
        lokasi = (body.get('lokasi') || '').trim();
        wilayah = (body.get('wilayah') || 'Kota Bandar Lampung').trim();
        latitude = body.get('latitude') ? parseFloat(body.get('latitude')) : null;
        longitude = body.get('longitude') ? parseFloat(body.get('longitude')) : null;
        fotoFile = body.get('foto');
      } else {
        kategori_id = parseInt(body.kategori_id || '1', 10);
        deskripsi = (body.deskripsi || '').trim();
        lokasi = (body.lokasi || '').trim();
        wilayah = (body.wilayah || 'Kota Bandar Lampung').trim();
        latitude = body.latitude ? parseFloat(body.latitude) : null;
        longitude = body.longitude ? parseFloat(body.longitude) : null;
        fotoFile = body.foto;
      }

      if (deskripsi.length < 10) {
        throw { status: 422, error: { details: [{ field: 'deskripsi', message: 'Deskripsi minimal 10 karakter.' }] } };
      }
      if (!lokasi) {
        throw { status: 422, error: { details: [{ field: 'lokasi', message: 'Lokasi wajib diisi.' }] } };
      }

      // Handle foto: jika upload file asli, jadikan base64
      let fotoUrl = null;
      if (fotoFile && fotoFile instanceof File && fotoFile.size > 0) {
        fotoUrl = await fileToDataUrl(fotoFile);
      } else if (typeof fotoFile === 'string' && fotoFile.length > 0) {
        fotoUrl = fotoFile;
      } else {
        // Foto default ilustrasi air jernih / pipa PDAM
        fotoUrl = 'https://images.unsplash.com/photo-1541888946425-d0fbb186f5f7?auto=format&fit=crop&w=800&q=80';
      }

      const category = db.categories.find((c) => c.id === kategori_id) || db.categories[0];
      const today = new Date().toISOString().slice(0, 10).replace(/-/g, '');
      const countToday = db.reports.filter((r) => (r.created_at || '').slice(0, 10).replace(/-/g, '') === today).length + 1;
      const nomor = `LAP-${today}-${String(countToday).padStart(4, '0')}`;
      const nextId = db.reports.length ? Math.max(...db.reports.map((r) => r.id)) + 1 : 1;
      const nowIso = new Date().toISOString();

      const newReport = {
        id: nextId,
        nomor,
        user_id: cur ? cur.id : 2,
        user_nama: cur ? cur.nama : 'Budi Santoso',
        category_id: category.id,
        kategori: category.nama,
        deskripsi,
        lokasi,
        wilayah,
        latitude,
        longitude,
        status: 'menunggu',
        foto: fotoUrl,
        created_at: nowIso,
        riwayat: [
          {
            status: 'menunggu',
            keterangan: 'Laporan masuk dari masyarakat',
            updated_by_nama: null,
            updated_at: nowIso,
          },
        ],
      };

      db.reports.unshift(newReport);
      saveDB(db);

      return {
        id: newReport.id,
        nomor: newReport.nomor,
        status: newReport.status,
        message: 'Laporan berhasil dibuat',
      };
    }

    // 8. GET /admin/reports (Semua laporan untuk admin)
    if (method === 'GET' && (cleanPath === '/admin/reports')) {
      let list = [...db.reports];

      const statusFilter = urlParams.get('status');
      if (statusFilter) list = list.filter((r) => r.status === statusFilter);

      const catFilter = urlParams.get('category_id');
      if (catFilter) list = list.filter((r) => String(r.category_id) === String(catFilter));

      const wilFilter = urlParams.get('wilayah');
      if (wilFilter) list = list.filter((r) => r.wilayah === wilFilter);

      const q = (urlParams.get('search') || urlParams.get('q') || '').trim().toLowerCase();
      if (q) {
        list = list.filter(
          (r) =>
            (r.nomor || '').toLowerCase().includes(q) ||
            (r.user_nama || '').toLowerCase().includes(q) ||
            (r.lokasi || '').toLowerCase().includes(q) ||
            (r.deskripsi || '').toLowerCase().includes(q)
        );
      }

      list.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));

      const page = parseInt(urlParams.get('page') || '1', 10);
      const per_page = parseInt(urlParams.get('per_page') || '10', 10);
      const total = list.length;
      const total_pages = Math.ceil(total / per_page) || 1;
      const start = (page - 1) * per_page;
      const paged = list.slice(start, start + per_page);

      return {
        data: paged,
        meta: { total, page, per_page, total_pages },
      };
    }

    // 9. GET /reports/:id atau /admin/reports/:id
    const reportMatch = cleanPath.match(/^\/(?:admin\/)?reports\/([^/]+)$/);
    if (method === 'GET' && reportMatch) {
      const rawId = decodeURIComponent(reportMatch[1]);
      const report = db.reports.find((r) => String(r.id) === rawId || r.nomor === rawId);
      if (!report) {
        throw { status: 404, error: { message: 'Laporan tidak ditemukan.' } };
      }
      return report;
    }

    // 10. PATCH /admin/reports/:id/status
    const statusMatch = cleanPath.match(/^\/admin\/reports\/([^/]+)\/status$/);
    if (method === 'PATCH' && statusMatch) {
      const rawId = decodeURIComponent(statusMatch[1]);
      const report = db.reports.find((r) => String(r.id) === rawId || r.nomor === rawId);
      if (!report) {
        throw { status: 404, error: { message: 'Laporan tidak ditemukan.' } };
      }
      const cur = getCurrentUser();
      const nextStatus = body.status;
      const ket = body.keterangan || (nextStatus === 'diproses' ? 'Laporan sedang ditangani petugas teknis' : 'Penanganan laporan telah selesai');
      const nowIso = new Date().toISOString();

      report.status = nextStatus;
      report.riwayat = report.riwayat || [];
      report.riwayat.push({
        status: nextStatus,
        keterangan: ket,
        updated_by_nama: cur ? cur.nama : 'Administrator PDAM Lampung',
        updated_at: nowIso,
      });

      saveDB(db);
      return { message: 'Status laporan berhasil diperbarui', status: nextStatus };
    }

    // Default 404
    throw { status: 404, error: { message: `Endpoint ${method} ${cleanPath} tidak ditemukan.` } };
  }

  // Ekspor window.api
  window.api = {
    get: (path, opts) => mockRouter('GET', path, {}),
    post: (path, body, opts) => mockRouter('POST', path, body),
    patch: (path, body, opts) => mockRouter('PATCH', path, body),
    getToken,
    setToken,
    getUser,
    setUser,
    clearSession,
    resetSimulationData,
    getDB,
  };
})();
