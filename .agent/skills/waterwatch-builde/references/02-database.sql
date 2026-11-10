-- ============================================================
-- WaterWatch Database Schema
-- Sumber: PRD v1.0 bagian 11.2
-- PostgreSQL 16
-- ============================================================

-- ── Migrasi 0001: Skema utama ──────────────────────────────

-- ENUM types
CREATE TYPE user_role AS ENUM ('masyarakat', 'admin');
CREATE TYPE report_status AS ENUM ('menunggu', 'diproses', 'selesai');

-- Tabel users
CREATE TABLE IF NOT EXISTS users (
    id            UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    nama          VARCHAR(100) NOT NULL,
    email         VARCHAR(150) NOT NULL UNIQUE,
    password_hash TEXT         NOT NULL,
    role          user_role    NOT NULL DEFAULT 'masyarakat',
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);

-- Tabel categories
CREATE TABLE IF NOT EXISTS categories (
    id             SMALLSERIAL  PRIMARY KEY,
    nama_kategori  VARCHAR(80)  NOT NULL UNIQUE
);

-- Tabel counter nomor laporan harian (format WW-YYYYMMDD-NNNN)
-- Satu baris per tanggal; counter di-increment atomically dalam transaksi
CREATE TABLE IF NOT EXISTS report_number_seq (
    tanggal DATE         PRIMARY KEY,
    counter INTEGER      NOT NULL DEFAULT 0
);

-- Tabel reports
CREATE TABLE IF NOT EXISTS reports (
    id          UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    nomor       VARCHAR(30)   NOT NULL UNIQUE,         -- WW-YYYYMMDD-NNNN
    user_id     UUID          NOT NULL REFERENCES users(id),
    category_id SMALLINT      NOT NULL REFERENCES categories(id),
    deskripsi   TEXT          NOT NULL
                              CHECK (char_length(deskripsi) BETWEEN 10 AND 1000),
    lokasi      TEXT          NOT NULL,
    latitude    DOUBLE PRECISION
                              CHECK (latitude IS NULL OR latitude BETWEEN -90 AND 90),
    longitude   DOUBLE PRECISION
                              CHECK (longitude IS NULL OR longitude BETWEEN -180 AND 180),
    wilayah     VARCHAR(100),
    foto        TEXT,                                   -- path relatif UUID, nullable
    status      report_status NOT NULL DEFAULT 'menunggu',
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ   NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_reports_user     ON reports(user_id,    created_at DESC);
CREATE INDEX IF NOT EXISTS idx_reports_status   ON reports(status,     created_at DESC);
CREATE INDEX IF NOT EXISTS idx_reports_wilayah  ON reports(wilayah);
CREATE INDEX IF NOT EXISTS idx_reports_category ON reports(category_id);

-- Tabel report_updates (riwayat setiap perubahan status)
CREATE TABLE IF NOT EXISTS report_updates (
    id          BIGSERIAL     PRIMARY KEY,
    report_id   UUID          NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    status      report_status NOT NULL,
    keterangan  TEXT,                                   -- keterangan wajib untuk update admin
    updated_by  UUID          REFERENCES users(id),    -- NULL jika sistem (pembuatan awal)
    updated_at  TIMESTAMPTZ   NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_updates_report ON report_updates(report_id, updated_at);

-- Tabel announcements (Fase 2 – tabel dibuat, endpoint tidak)
CREATE TABLE IF NOT EXISTS announcements (
    id          BIGSERIAL    PRIMARY KEY,
    judul       VARCHAR(150) NOT NULL,
    isi         TEXT         NOT NULL,
    wilayah     VARCHAR(100) NOT NULL,
    tanggal     DATE         NOT NULL DEFAULT CURRENT_DATE,
    created_by  UUID         REFERENCES users(id)
);

-- ── Migrasi 0002: Seed kategori ───────────────────────────

INSERT INTO categories (nama_kategori) VALUES
    ('Kebocoran Pipa'),
    ('Air Tidak Mengalir'),
    ('Kualitas Air Bermasalah'),
    ('Lainnya')
ON CONFLICT (nama_kategori) DO NOTHING;

-- ============================================================
-- CARA GENERATE NOMOR LAPORAN (implementasi di Rust)
-- ============================================================
-- Dalam satu transaksi DB:
--   1. INSERT INTO report_number_seq (tanggal, counter)
--      VALUES (CURRENT_DATE, 1)
--      ON CONFLICT (tanggal) DO UPDATE
--        SET counter = report_number_seq.counter + 1
--      RETURNING counter;
--   2. Format: format!("WW-{}-{:04}", today.format("%Y%m%d"), counter)
--   3. INSERT INTO reports (nomor, ...) VALUES (...);
--   4. INSERT INTO report_updates (report_id, status, keterangan, updated_by, updated_at) ...;
--   5. COMMIT;
--
-- Jika counter = 1 → WW-20261110-0001
-- Jika counter = 42 → WW-20261110-0042
-- ============================================================

