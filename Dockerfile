# ============================================================
# WaterWatch – Multi-Stage Dockerfile
# Stage 1 (builder): Kompilasi Rust → binary
# Stage 2 (runner): Jalankan binary di image minimal
#
# PENTING: Postgres harus berjalan sebelum menjalankan
# `docker compose build` karena SQLx memverifikasi query
# ke database saat kompilasi.
# Jalankan dulu: docker compose up -d postgres
#                docker compose run --rm migrate
# Lalu: docker compose build
# ============================================================

# ── Stage 1: Builder ────────────────────────────────────────
# Gunakan Rust stable terbaru agar kompatibel dengan semua crate
FROM rust:latest AS builder

# Install dependensi build (OpenSSL, pkg-config, dll.)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Salin Cargo.toml terlebih dahulu
# (agar layer cache dependensi tidak diulang jika hanya src berubah)
COPY Cargo.toml ./

# Buat dummy main agar `cargo build --release` bisa cache dependensi
# (Tahap ini TIDAK butuh DATABASE_URL karena tidak ada sqlx macro)
RUN mkdir -p src/bin && \
    echo 'fn main() {}' > src/main.rs && \
    echo 'fn main() {}' > src/bin/seed.rs && \
    cargo build --release && \
    rm -rf src

# ── DATABASE_URL dibutuhkan mulai sini ───────────────────────
# SQLx memverifikasi query SQL ke database saat kompilasi.
# host.docker.internal mengarah ke host Windows/Mac dari dalam container.
# Postgres harus sudah berjalan di host:5432 sebelum docker compose build.
ARG DATABASE_URL=postgres://waterwatch:waterwatch_secret@host.docker.internal:5432/waterwatch
ENV DATABASE_URL=${DATABASE_URL}

# Salin source code asli dan tests, lalu build ulang
COPY src ./src
COPY tests ./tests

# Sentuh main.rs agar cargo tahu ada perubahan
RUN touch src/main.rs src/bin/seed.rs && \
    cargo build --release

# ── Stage 2: Runner ─────────────────────────────────────────
FROM debian:bookworm-slim AS runner

# Install runtime libs minimal
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Buat user non-root untuk keamanan
RUN useradd --uid 1000 --no-create-home --shell /bin/false waterwatch

WORKDIR /app

# Salin binary dari stage builder
COPY --from=builder /build/target/release/waterwatch /app/waterwatch
COPY --from=builder /build/target/release/seed       /app/seed

# Buat folder uploads dengan permission yang benar
RUN mkdir -p /app/uploads && chown -R waterwatch:waterwatch /app

# Salin file static frontend
COPY static ./static

USER waterwatch

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget -qO- http://localhost:8080/health || exit 1

CMD ["/app/waterwatch"]

