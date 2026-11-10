// static/js/layout.js – Muat footer di semua halaman

async function loadFooter() {
  // Support dua ID untuk kompatibilitas semua halaman
  const c = document.getElementById('ww-footer-container')
         || document.getElementById('footer-placeholder');
  if (!c) return;
  try {
    const res = await fetch('/static/partials/footer.html');
    if (res.ok) c.innerHTML = await res.text(); // footer.html = konten statis, bukan data pengguna
    else throw new Error('fetch gagal');
  } catch {
    // Fallback minimal jika footer.html tidak bisa dimuat
    c.innerHTML = `<footer class="ww-footer text-center py-3 border-top">
      <p class="mb-1 small">Copyright &copy; 2026 by Aris Triantoro</p>
      <p class="mb-0 small">Dr. Erlyan Redy Susanto, M.Kom</p>
    </footer>`;
  }
}

// Tandai nav item aktif berdasarkan path saat ini
function markActiveNav() {
  const path = window.location.pathname;
  document.querySelectorAll('.ww-nav-item, .ww-sidebar-link').forEach(el => {
    const href = el.getAttribute('href') || '';
    if (href && path.endsWith(href)) el.classList.add('active');
  });
}

document.addEventListener('DOMContentLoaded', () => {
  loadFooter();
  markActiveNav();
});

