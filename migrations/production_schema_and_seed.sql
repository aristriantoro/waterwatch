-- ============================================================
-- WaterWatch Provinsi Lampung – Skema & Seed Database Produksi
-- Kompatibel dengan: Neon.tech, Supabase, Render Postgres, AWS RDS
-- Jalankan skrip ini langsung di SQL Editor platform database Anda.
-- ============================================================

-- 1. Ekstensi UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 2. Tipe Data Enum
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('masyarakat', 'admin');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE report_status AS ENUM ('menunggu', 'diproses', 'selesai');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 3. Tabel Pengguna (users)
CREATE TABLE IF NOT EXISTS users (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nama          VARCHAR(100) NOT NULL,
    email         VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role          user_role NOT NULL DEFAULT 'masyarakat',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. Tabel Kategori Gangguan Air (categories)
CREATE TABLE IF NOT EXISTS categories (
    id            SMALLSERIAL PRIMARY KEY,
    nama_kategori VARCHAR(50) NOT NULL UNIQUE
);

-- 5. Tabel Counter Urutan Laporan Harian (WW-YYYYMMDD-NNNN)
CREATE TABLE IF NOT EXISTS report_number_seq (
    tanggal DATE PRIMARY KEY,
    counter INT NOT NULL DEFAULT 1
);

-- 6. Tabel Laporan Gangguan (reports)
CREATE TABLE IF NOT EXISTS reports (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nomor       VARCHAR(20) NOT NULL UNIQUE,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    category_id SMALLINT NOT NULL REFERENCES categories(id) ON DELETE RESTRICT,
    deskripsi   TEXT NOT NULL,
    lokasi      VARCHAR(255) NOT NULL,
    latitude    DECIMAL(10, 8),
    longitude   DECIMAL(11, 8),
    wilayah     VARCHAR(100),
    foto        VARCHAR(255),
    status      report_status NOT NULL DEFAULT 'menunggu',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 7. Tabel Riwayat Audit Perubahan Status (report_updates)
CREATE TABLE IF NOT EXISTS report_updates (
    id         BIGSERIAL PRIMARY KEY,
    report_id  UUID NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    status     report_status NOT NULL,
    keterangan TEXT,
    updated_by UUID REFERENCES users(id) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 8. Tabel Pengumuman (announcements - untuk kesiapan rilis lanjutan)
CREATE TABLE IF NOT EXISTS announcements (
    id         SERIAL PRIMARY KEY,
    judul      VARCHAR(200) NOT NULL,
    isi        TEXT NOT NULL,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 9. Indeks Performa
CREATE INDEX IF NOT EXISTS idx_reports_user_id     ON reports(user_id);
CREATE INDEX IF NOT EXISTS idx_reports_status      ON reports(status);
CREATE INDEX IF NOT EXISTS idx_reports_category_id ON reports(category_id);
CREATE INDEX IF NOT EXISTS idx_reports_created_at  ON reports(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_report_updates_report_id ON report_updates(report_id, updated_at ASC);

-- 10. Data Awal Kategori Gangguan Air
INSERT INTO categories (id, nama_kategori) VALUES
    (1, 'Pipa Bocor'),
    (2, 'Air Keruh / Berbau'),
    (3, 'Air Mati Total'),
    (4, 'Tekanan Air Rendah')
ON CONFLICT (id) DO UPDATE SET nama_kategori = EXCLUDED.nama_kategori;

SELECT setval('categories_id_seq', (SELECT MAX(id) FROM categories));

-- 11. Seed Akun Admin (Password: Admin@1234!)
INSERT INTO users (nama, email, password_hash, role) VALUES
    ('Admin WaterWatch 1', 'admin@waterwatch.id',  '$argon2id$v=19$m=19456,t=2,p=1$C946CUN0EuAgNLAZKfB6Bg$Cx7HY1s3nWoXh7zicciAiwA9AE503Y8L1v0DG1KL45M', 'admin'),
    ('Admin WaterWatch 2', 'admin2@waterwatch.id', '$argon2id$v=19$m=19456,t=2,p=1$cCwdx5iAwwBfPy6s5Bp2oQ$+XbkSb9jggJnA8b6tA5jgoLlyetrcUfl0n1J/IfQQOQ', 'admin')
ON CONFLICT (email) DO NOTHING;

-- 12. Seed Akun Warga Contoh (Password: Warga@1234!)
INSERT INTO users (nama, email, password_hash, role) VALUES
    ('Budi Santoso',    'warga1@example.com',  '$argon2id$v=19$m=19456,t=2,p=1$c/ESgx/SusvbYMiQ7z3biw$00f4ov1KsaEwU1YEj4t73dVqQTgJNaixxtjYlk6ysmM', 'masyarakat'),
    ('Siti Rahayu',     'warga2@example.com',  '$argon2id$v=19$m=19456,t=2,p=1$c/ESgx/SusvbYMiQ7z3biw$00f4ov1KsaEwU1YEj4t73dVqQTgJNaixxtjYlk6ysmM', 'masyarakat'),
    ('Ahmad Fauzi',     'warga3@example.com',  '$argon2id$v=19$m=19456,t=2,p=1$c/ESgx/SusvbYMiQ7z3biw$00f4ov1KsaEwU1YEj4t73dVqQTgJNaixxtjYlk6ysmM', 'masyarakat'),
    ('Dewi Lestari',    'warga4@example.com',  '$argon2id$v=19$m=19456,t=2,p=1$c/ESgx/SusvbYMiQ7z3biw$00f4ov1KsaEwU1YEj4t73dVqQTgJNaixxtjYlk6ysmM', 'masyarakat'),
    ('Hendra Gunawan',  'warga5@example.com',  '$argon2id$v=19$m=19456,t=2,p=1$c/ESgx/SusvbYMiQ7z3biw$00f4ov1KsaEwU1YEj4t73dVqQTgJNaixxtjYlk6ysmM', 'masyarakat')
ON CONFLICT (email) DO NOTHING;

