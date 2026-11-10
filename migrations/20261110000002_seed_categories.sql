-- ============================================================
-- Migrasi 0002: Seed Kategori Awal
-- Sumber: PRD v1.0 FR-08
-- ============================================================

INSERT INTO categories (nama_kategori) VALUES
    ('Kebocoran Pipa'),
    ('Air Tidak Mengalir'),
    ('Kualitas Air Bermasalah'),
    ('Lainnya')
ON CONFLICT (nama_kategori) DO NOTHING;

