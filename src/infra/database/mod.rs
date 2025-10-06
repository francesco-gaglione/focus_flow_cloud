pub mod migrations;
pub mod persistence;

use crate::infra::database::migrations::MIGRATIONS;
use deadpool_diesel::postgres::Pool;
use tracing::info;

pub async fn setup_database() -> Pool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);

    Pool::builder(manager)
        .build()
        .expect("Failed to create database pool")
}

pub async fn run_migrations(pool: &Pool) {
    use diesel_migrations::MigrationHarness;

    let conn = pool.get().await.expect("Failed to get database connection");

    conn.interact(|conn| {
        conn.run_pending_migrations(MIGRATIONS)
            .map(|_| ())
            .expect("Failed to run migrations")
    })
    .await
    .expect("Migration interaction failed");

    info!("Database migrations completed successfully");
}
