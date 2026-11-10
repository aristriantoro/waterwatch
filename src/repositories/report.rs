// src/repositories/report.rs – Query DB untuk tabel reports & report_updates

use chrono::Utc;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        report::{
            AdminReportQuery, CreateReportResponse, ReportDetail,
            ReportQuery, ReportStatus, ReportSummary, ReportSummaryAdmin,
            UpdateStatusResponse,
        },
        report_update::ReportUpdateResponse,
    },
};

/// Generate nomor laporan harian secara atomis dalam transaksi
/// Format: WW-YYYYMMDD-NNNN
pub async fn generate_report_number(
    tx: &mut Transaction<'_, Postgres>,
) -> Result<String, AppError> {
    let today = Utc::now().date_naive();
    let row = sqlx::query!(
        r#"
        INSERT INTO report_number_seq (tanggal, counter)
        VALUES ($1, 1)
        ON CONFLICT (tanggal) DO UPDATE
            SET counter = report_number_seq.counter + 1
        RETURNING counter
        "#,
        today
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(AppError::Database)?;

    Ok(format!("WW-{}-{:04}", today.format("%Y%m%d"), row.counter))
}

/// Buat laporan baru – SATU transaksi (insert report + insert report_updates)
pub async fn create_report(
    pool: &PgPool,
    user_id: Uuid,
    category_id: i16,
    deskripsi: &str,
    lokasi: &str,
    latitude: Option<f64>,
    longitude: Option<f64>,
    wilayah: Option<&str>,
    foto: Option<&str>,
) -> Result<CreateReportResponse, AppError> {
    let mut tx = pool.begin().await.map_err(AppError::Database)?;

    let nomor = generate_report_number(&mut tx).await?;
    let now = Utc::now();

    let report = sqlx::query!(
        r#"
        INSERT INTO reports (nomor, user_id, category_id, deskripsi, lokasi,
                             latitude, longitude, wilayah, foto, status,
                             created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'menunggu', $10, $10)
        RETURNING id, nomor, status AS "status: ReportStatus", created_at
        "#,
        nomor,
        user_id,
        category_id,
        deskripsi,
        lokasi,
        latitude,
        longitude,
        wilayah,
        foto,
        now,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    // Wajib: satu entri report_updates saat laporan dibuat
    sqlx::query!(
        r#"
        INSERT INTO report_updates (report_id, status, keterangan, updated_by, updated_at)
        VALUES ($1, 'menunggu', 'Laporan masuk', NULL, $2)
        "#,
        report.id,
        now,
    )
    .execute(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    tx.commit().await.map_err(AppError::Database)?;

    Ok(CreateReportResponse {
        id: report.id,
        nomor: report.nomor,
        status: report.status,
        created_at: report.created_at,
    })
}

/// Daftar laporan milik warga (dengan pagination & filter)
pub async fn find_mine(
    pool: &PgPool,
    user_id: Uuid,
    q: &ReportQuery,
) -> Result<(Vec<ReportSummary>, i64), AppError> {
    let offset = ((q.page.saturating_sub(1)) * q.per_page) as i64;
    let per_page = q.per_page as i64;

    let rows = sqlx::query_as!(
        ReportSummary,
        r#"
        SELECT r.id, r.nomor,
               c.nama_kategori AS "kategori",
               r.deskripsi, r.lokasi,
               r.status AS "status: ReportStatus",
               r.created_at, r.updated_at
        FROM reports r
        JOIN categories c ON c.id = r.category_id
        WHERE r.user_id = $1
          AND ($2::report_status IS NULL OR r.status = $2)
        ORDER BY r.created_at DESC
        LIMIT $3 OFFSET $4
        "#,
        user_id,
        q.status as Option<ReportStatus>,
        per_page,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    let total: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM reports
        WHERE user_id = $1
          AND ($2::report_status IS NULL OR status = $2)
        "#,
        user_id,
        q.status as Option<ReportStatus>,
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?
    .unwrap_or(0);

    Ok((rows, total))
}

/// Detail laporan (pemilik atau admin)
pub async fn find_detail_by_id(
    pool: &PgPool,
    report_id: Uuid,
) -> Result<Option<ReportDetail>, AppError> {
    let row = sqlx::query!(
        r#"
        SELECT r.id, r.nomor,
               u.nama AS "user_nama?: String",
               c.nama_kategori AS "kategori",
               r.deskripsi, r.lokasi, r.latitude, r.longitude,
               r.wilayah, r.foto,
               r.status AS "status: ReportStatus",
               r.created_at, r.updated_at
        FROM reports r
        JOIN categories c ON c.id = r.category_id
        JOIN users u ON u.id = r.user_id
        WHERE r.id = $1
        "#,
        report_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    let Some(r) = row else { return Ok(None) };

    let riwayat = sqlx::query_as!(
        ReportUpdateResponse,
        r#"
        SELECT ru.status AS "status: ReportStatus",
               ru.keterangan,
               u.nama AS "updated_by_nama?: String",
               ru.updated_at
        FROM report_updates ru
        LEFT JOIN users u ON u.id = ru.updated_by
        WHERE ru.report_id = $1
        ORDER BY ru.updated_at ASC
        "#,
        report_id,
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(Some(ReportDetail {
        id: r.id,
        nomor: r.nomor,
        user_nama: r.user_nama,
        kategori: r.kategori,
        deskripsi: r.deskripsi,
        lokasi: r.lokasi,
        latitude: r.latitude,
        longitude: r.longitude,
        wilayah: r.wilayah,
        foto: r.foto,
        status: r.status,
        created_at: r.created_at,
        updated_at: r.updated_at,
        riwayat,
    }))
}

/// Cek apakah laporan dimiliki oleh user tertentu
pub async fn get_report_owner(pool: &PgPool, report_id: Uuid) -> Result<Option<Uuid>, AppError> {
    let row = sqlx::query_scalar!("SELECT user_id FROM reports WHERE id = $1", report_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(row)
}

/// Daftar semua laporan untuk admin (dengan filter & pagination)
pub async fn find_all_admin(
    pool: &PgPool,
    q: &AdminReportQuery,
) -> Result<(Vec<ReportSummaryAdmin>, i64), AppError> {
    let offset = ((q.page.saturating_sub(1)) * q.per_page) as i64;
    let per_page = q.per_page as i64;
    let search = q.q.as_deref().map(|s| format!("%{s}%"));

    let rows = sqlx::query_as!(
        ReportSummaryAdmin,
        r#"
        SELECT r.id, r.nomor,
               u.nama AS "user_nama",
               c.nama_kategori AS "kategori",
               r.deskripsi, r.lokasi,
               r.status AS "status: ReportStatus",
               r.created_at, r.updated_at
        FROM reports r
        JOIN categories c ON c.id = r.category_id
        JOIN users u ON u.id = r.user_id
        WHERE ($1::report_status IS NULL OR r.status = $1)
          AND ($2::smallint IS NULL OR r.category_id = $2)
          AND ($3::date IS NULL OR r.created_at::date >= $3)
          AND ($4::date IS NULL OR r.created_at::date <= $4)
          AND ($5::text IS NULL OR r.deskripsi ILIKE $5 OR r.lokasi ILIKE $5)
        ORDER BY r.created_at DESC
        LIMIT $6 OFFSET $7
        "#,
        q.status as Option<ReportStatus>,
        q.kategori_id,
        q.tanggal_dari,
        q.tanggal_sampai,
        search,
        per_page,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    let total: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM reports r
        WHERE ($1::report_status IS NULL OR r.status = $1)
          AND ($2::smallint IS NULL OR r.category_id = $2)
          AND ($3::date IS NULL OR r.created_at::date >= $3)
          AND ($4::date IS NULL OR r.created_at::date <= $4)
          AND ($5::text IS NULL OR r.deskripsi ILIKE $5 OR r.lokasi ILIKE $5)
        "#,
        q.status as Option<ReportStatus>,
        q.kategori_id,
        q.tanggal_dari,
        q.tanggal_sampai,
        search,
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?
    .unwrap_or(0);

    Ok((rows, total))
}

/// Ubah status laporan – SATU transaksi (update reports + insert report_updates)
pub async fn update_status(
    pool: &PgPool,
    report_id: Uuid,
    new_status: ReportStatus,
    keterangan: &str,
    updated_by: Uuid,
) -> Result<UpdateStatusResponse, AppError> {
    let mut tx = pool.begin().await.map_err(AppError::Database)?;
    let now = Utc::now();

    let updated = sqlx::query!(
        r#"
        UPDATE reports SET status = $1, updated_at = $2
        WHERE id = $3
        RETURNING id, status AS "status: ReportStatus", updated_at
        "#,
        new_status.clone() as ReportStatus,
        now,
        report_id,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    sqlx::query!(
        r#"
        INSERT INTO report_updates (report_id, status, keterangan, updated_by, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        report_id,
        new_status as ReportStatus,
        keterangan,
        updated_by,
        now,
    )
    .execute(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    tx.commit().await.map_err(AppError::Database)?;

    Ok(UpdateStatusResponse {
        id: updated.id,
        status: updated.status,
        updated_at: updated.updated_at,
    })
}

/// Statistik untuk dashboard admin
pub struct DashboardStats {
    pub total:        i64,
    pub menunggu:     i64,
    pub diproses:     i64,
    pub selesai:      i64,
    pub hari_ini:     i64,
}

pub async fn get_dashboard_stats(pool: &PgPool) -> Result<DashboardStats, AppError> {
    let row = sqlx::query!(
        r#"
        SELECT
            COUNT(*) AS "total!",
            COUNT(*) FILTER (WHERE status = 'menunggu') AS "menunggu!",
            COUNT(*) FILTER (WHERE status = 'diproses') AS "diproses!",
            COUNT(*) FILTER (WHERE status = 'selesai')  AS "selesai!",
            COUNT(*) FILTER (WHERE created_at::date = CURRENT_DATE) AS "hari_ini!"
        FROM reports
        "#
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(DashboardStats {
        total:    row.total,
        menunggu: row.menunggu,
        diproses: row.diproses,
        selesai:  row.selesai,
        hari_ini: row.hari_ini,
    })
}

pub struct CategoryStat {
    pub kategori: String,
    pub jumlah:   i64,
}

pub async fn get_category_stats(pool: &PgPool) -> Result<Vec<CategoryStat>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT c.nama_kategori AS "kategori!", COUNT(r.id) AS "jumlah!"
        FROM categories c
        LEFT JOIN reports r ON r.category_id = c.id
        GROUP BY c.id, c.nama_kategori
        ORDER BY c.id
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(rows.into_iter().map(|r| CategoryStat { kategori: r.kategori, jumlah: r.jumlah }).collect())
}

pub async fn find_latest(pool: &PgPool, limit: i64) -> Result<Vec<ReportSummaryAdmin>, AppError> {
    let rows = sqlx::query_as!(
        ReportSummaryAdmin,
        r#"
        SELECT r.id, r.nomor,
               u.nama AS "user_nama",
               c.nama_kategori AS "kategori",
               r.deskripsi, r.lokasi,
               r.status AS "status: ReportStatus",
               r.created_at, r.updated_at
        FROM reports r
        JOIN categories c ON c.id = r.category_id
        JOIN users u ON u.id = r.user_id
        ORDER BY r.created_at DESC
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(rows)
}

