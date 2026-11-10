const fs = require('fs');
const path = require('path');

const rootDir = __dirname;
const staticDir = path.join(rootDir, 'static');
const publicDir = path.join(rootDir, 'public');

console.log('--- WaterWatch Static Build ---');

// Bersihkan folder public jika ada
if (fs.existsSync(publicDir)) {
  fs.rmSync(publicDir, { recursive: true, force: true });
}
fs.mkdirSync(publicDir, { recursive: true });

// 1. Salin seluruh isi static/ ke public/ (agar root seperti /index.html, /warga/, /admin/ tersedia)
fs.cpSync(staticDir, publicDir, { recursive: true });

// 2. Salin static/ ke public/static/ (agar asset seperti /static/css/..., /static/js/... tetap berjalan)
const publicStaticDir = path.join(publicDir, 'static');
fs.cpSync(staticDir, publicStaticDir, { recursive: true });

console.log('Build berhasil! public/ terisi file HTML, CSS, JS, dan assets.');

