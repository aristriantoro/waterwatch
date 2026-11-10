// src/bin/seed.rs – Seed admin dan data simulasi
// Jalankan: docker compose run --rm app /app/seed
// PERINGATAN: Jangan jalankan di produksi yang sudah ada data!

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL harus diset di .env");
    let pool = PgPool::connect(&database_url).await?;

    seed_admins(&pool).await?;
    let warga_ids = seed_warga(&pool).await?;
    seed_reports(&pool, &warga_ids).await?;

    tracing::info!("✅ Seed selesai!");
    tracing::info!("   Admin 1: admin@waterwatch.id / Admin@1234!");
    tracing::info!("   Admin 2: admin2@waterwatch.id / Admin@1234!");
    tracing::info!("   Warga  : warga1@example.com s.d. warga10@example.com / Warga@1234!");
    Ok(())
}

fn hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("hash gagal")
        .to_string()
}

async fn seed_admins(pool: &PgPool) -> anyhow::Result<()> {
    let admins = [
        ("Admin WaterWatch",   "admin@waterwatch.id"),
        ("Admin WaterWatch 2", "admin2@waterwatch.id"),
    ];
    for (nama, email) in admins {
        let ph = hash("Admin@1234!");
        sqlx::query(
            r#"INSERT INTO users (nama, email, password_hash, role)
               VALUES ($1, $2, $3, 'admin')
               ON CONFLICT (email) DO NOTHING"#,
        )
        .bind(nama)
        .bind(email)
        .bind(ph)
        .execute(pool)
        .await?;
        tracing::info!("Admin seeded: {email}");
    }
    Ok(())
}

async fn seed_warga(pool: &PgPool) -> anyhow::Result<Vec<Uuid>> {
    let mut ids = vec![];
    let nama_list = [
        ("Budi Santoso",    "warga1@example.com"),
        ("Siti Rahayu",     "warga2@example.com"),
        ("Ahmad Fauzi",     "warga3@example.com"),
        ("Dewi Lestari",    "warga4@example.com"),
        ("Hendra Gunawan",  "warga5@example.com"),
        ("Rina Wulandari",  "warga6@example.com"),
        ("Dodi Firmansyah", "warga7@example.com"),
        ("Yuni Astuti",     "warga8@example.com"),
        ("Surya Pratama",   "warga9@example.com"),
        ("Maya Sari",       "warga10@example.com"),
        ("Fajar Nugroho",   "warga11@example.com"),
        ("Lina Marlina",    "warga12@example.com"),
    ];
    for (nama, email) in nama_list {
        let ph = hash("Warga@1234!");
        let row = sqlx::query_scalar::<_, Uuid>(
            r#"INSERT INTO users (nama, email, password_hash, role)
               VALUES ($1, $2, $3, 'masyarakat')
               ON CONFLICT (email) DO UPDATE SET nama = EXCLUDED.nama
               RETURNING id"#,
        )
        .bind(nama)
        .bind(email)
        .bind(ph)
        .fetch_one(pool)
        .await?;
        ids.push(row);
        tracing::info!("Warga seeded: {email}");
    }
    Ok(ids)
}

async fn seed_reports(pool: &PgPool, warga_ids: &[Uuid]) -> anyhow::Result<()> {
    let categories: Vec<i16> = sqlx::query_scalar("SELECT id FROM categories ORDER BY id")
        .fetch_all(pool)
        .await?;

    let lokasi_list = [
        "Jl. Raden Intan No. 15, Bandar Lampung",
        "Jl. Teuku Umar No. 7, Kedaton",
        "Jl. Pagar Alam No. 22, Labuhan Ratu",
        "Jl. Z.A. Pagar Alam No. 50, Rajabasa",
        "Jl. Ryacudu No. 3, Sukarame",
        "Jl. Sultan Agung No. 11, Tanjung Karang",
        "Jl. Urip Sumoharjo No. 8, Enggal",
        "Jl. Ahmad Yani No. 30, Teluk Betung",
        "Jl. Wolter Monginsidi No. 4, Pahoman",
        "Jl. Gatot Subroto No. 17, Way Halim",
    ];

    let deskripsi_list = [
        "Pipa utama bocor di depan rumah, air meluap ke jalan dan mengganggu lalu lintas.",
        "Air tidak mengalir sejak pagi hari, sudah 2 hari keran kering total.",
        "Air berwarna kecoklatan dan berbau, tidak layak digunakan untuk memasak.",
        "Kebocoran pipa besar di persimpangan jalan, air membasahi aspal seluas 10 meter.",
        "Aliran air sangat kecil, hanya menetes padahal biasanya normal.",
        "Air berbusa saat dialirkan, diduga ada kontaminasi sabun atau bahan kimia.",
        "Pipa bawah tanah seperti bocor, terlihat tanah basah padahal tidak hujan.",
        "Sudah 3 hari air mati total di RT 05, warga terpaksa beli air galon.",
        "Tekanan air sangat rendah, tidak bisa digunakan untuk mandi di lantai 2.",
        "Air keruh dan ada endapan pasir, sudah satu minggu seperti ini.",
    ];

    let wilayah_list = [
        "Kedaton", "Labuhan Ratu", "Rajabasa", "Sukarame", "Enggal",
        "Way Halim", "Teluk Betung", "Pahoman", "Tanjung Karang", "Panjang",
    ];

    let statuses = ["menunggu", "menunggu", "menunggu", "diproses", "diproses", "selesai"];

    // Ambil admin_id menggunakan dynamic query
    let admin_id: Uuid = sqlx::query_scalar(
        "SELECT id FROM users WHERE email = 'admin@waterwatch.id'",
    )
    .fetch_one(pool)
    .await?;

    let n = warga_ids.len();
    let mut report_count = 0i32;

    for i in 0..55usize {
        let warga_id = warga_ids[i % n];
        let cat_id   = categories[i % categories.len()];
        let lokasi   = lokasi_list[i % lokasi_list.len()];
        let deskripsi = format!("{} (laporan ke-{})", deskripsi_list[i % deskripsi_list.len()], i + 1);
        let wilayah  = wilayah_list[i % wilayah_list.len()];
        let status   = statuses[i % statuses.len()];

        // Tanggal dibuat bervariasi (1–30 hari lalu)
        let days_ago = (i % 30) as i64;
        let created  = Utc::now() - Duration::days(days_ago);
        let today    = created.date_naive();

        // Generate nomor laporan – dynamic query (counter untuk seed)
        let counter: i32 = sqlx::query_scalar(
            r#"INSERT INTO report_number_seq (tanggal, counter)
               VALUES ($1, 1)
               ON CONFLICT (tanggal) DO UPDATE
                   SET counter = report_number_seq.counter + 1
               RETURNING counter"#,
        )
        .bind(today)
        .fetch_one(pool)
        .await?;
        let nomor = format!("WW-{}-{:04}", today.format("%Y%m%d"), counter);

        // Insert report – gunakan dynamic query agar bisa cast ke report_status
        let report_id: Uuid = sqlx::query_scalar(
            r#"INSERT INTO reports (nomor, user_id, category_id, deskripsi, lokasi,
                                    wilayah, status, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7::report_status, $8, $8)
               RETURNING id"#,
        )
        .bind(&nomor)
        .bind(warga_id)
        .bind(cat_id)
        .bind(&deskripsi)
        .bind(lokasi)
        .bind(wilayah)
        .bind(status)
        .bind(created)
        .fetch_one(pool)
        .await?;

        // report_updates: status awal (menunggu)
        sqlx::query(
            "INSERT INTO report_updates (report_id, status, keterangan, updated_by, updated_at)
             VALUES ($1, 'menunggu', 'Laporan masuk', NULL, $2)",
        )
        .bind(report_id)
        .bind(created)
        .execute(pool)
        .await?;

        // Jika diproses atau selesai: tambah update
        if status == "diproses" || status == "selesai" {
            let t2 = created + Duration::hours(2);
            sqlx::query(
                r#"INSERT INTO report_updates (report_id, status, keterangan, updated_by, updated_at)
                   VALUES ($1, 'diproses', 'Laporan telah diverifikasi, tim sedang menangani.', $2, $3)"#,
            )
            .bind(report_id)
            .bind(admin_id)
            .bind(t2)
            .execute(pool)
            .await?;

            sqlx::query(
                "UPDATE reports SET status = 'diproses'::report_status, updated_at = $1 WHERE id = $2",
            )
            .bind(t2)
            .bind(report_id)
            .execute(pool)
            .await?;
        }

        if status == "selesai" {
            let t3 = created + Duration::hours(24);
            sqlx::query(
                r#"INSERT INTO report_updates (report_id, status, keterangan, updated_by, updated_at)
                   VALUES ($1, 'selesai', 'Perbaikan selesai, gangguan sudah teratasi.', $2, $3)"#,
            )
            .bind(report_id)
            .bind(admin_id)
            .bind(t3)
            .execute(pool)
            .await?;

            sqlx::query(
                "UPDATE reports SET status = 'selesai'::report_status, updated_at = $1 WHERE id = $2",
            )
            .bind(t3)
            .bind(report_id)
            .execute(pool)
            .await?;
        }

        report_count += 1;
        tracing::info!("Report seeded: {nomor} [{status}]");
    }

    tracing::info!("Total laporan: {report_count}");
    Ok(())
}
