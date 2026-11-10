-- ============================================================
-- Migrasi 0001: Skema Utama WaterWatch
-- PostgreSQL 16
-- Sumber: PRD v1.0 bagian 11.2
-- ============================================================

-- ENUM: role pengguna
CREATE TYPE user_role AS ENUM ('masyarakat', 'admin');

-- ENUM: status laporan
CREATE TYPE report_status AS ENUM ('menunggu', 'diproses', 'selesai');

-- ── Tabel users ───────────────────────────────────────────
CREATE TABLE IF NOT EXISTS users (
    id            UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    nama          VARCHAR(100) NOT NULL,
    email         VARCHAR(150) NOT NULL UNIQUE,
    password_hash TEXT         NOT NULL,
    role          user_role    NOT NULL DEFAULT 'masyarakat',
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);

-- ── Tabel categories ──────────────────────────────────────
CREATE TABLE IF NOT EXISTS categories (
    id             SMALLSERIAL  PRIMARY KEY,
    nama_kategori  VARCHAR(80)  NOT NULL UNIQUE
);

-- ── Tabel counter nomor laporan harian ────────────────────
-- Format nomor: WW-YYYYMMDD-NNNN (counter di-increment atomically)
CREATE TABLE IF NOT EXISTS report_number_seq (
    tanggal DATE    PRIMARY KEY,
    counter INTEGER NOT NULL DEFAULT 0
);

-- ── Tabel reports ─────────────────────────────────────────
CREATE TABLE IF NOT EXISTS reports (
    id          UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    nomor       VARCHAR(30)   NOT NULL UNIQUE,
    user_id     UUID          NOT NULL REFERENCES users(id),
    category_id SMALLINT      NOT NULL REFERENCES categories(id),
    deskripsi   TEXT          NOT NULL
                              CHECK (char_length(deskripsi) BETWEEN 10 AND 1000),
    lokasi      TEXT          NOT NULL,
    latitude    DOUBLE PRECISION
                              CHECK (latitude IS NULL OR (latitude BETWEEN -90 AND 90)),
    longitude   DOUBLE PRECISION
                              CHECK (longitude IS NULL OR (longitude BETWEEN -180 AND 180)),
    wilayah     VARCHAR(100),
    foto        TEXT,
    status      report_status NOT NULL DEFAULT 'menunggu',
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ   NOT NULL DEFAULT now()
);

CREATE INDEX idx_reports_user     ON reports(user_id,    created_at DESC);
CREATE INDEX idx_reports_status   ON reports(status,     created_at DESC);
CREATE INDEX idx_reports_wilayah  ON reports(wilayah);
CREATE INDEX idx_reports_category ON reports(category_id);

-- ── Tabel report_updates (riwayat perubahan status) ───────
CREATE TABLE IF NOT EXISTS report_updates (
    id          BIGSERIAL     PRIMARY KEY,
    report_id   UUID          NOT NULL REFERENCES reports(id) ON DELETE CASCADE,
    status      report_status NOT NULL,
    keterangan  TEXT,
    updated_by  UUID          REFERENCES users(id),
    updated_at  TIMESTAMPTZ   NOT NULL DEFAULT now()
);

CREATE INDEX idx_updates_report ON report_updates(report_id, updated_at);

-- ── Tabel announcements (Fase 2 – tabel dibuat, endpoint tidak) ───
CREATE TABLE IF NOT EXISTS announcements (
    id          BIGSERIAL    PRIMARY KEY,
    judul       VARCHAR(150) NOT NULL,
    isi         TEXT         NOT NULL,
    wilayah     VARCHAR(100) NOT NULL,
    tanggal     DATE         NOT NULL DEFAULT CURRENT_DATE,
    created_by  UUID         REFERENCES users(id)
);

