pub mod migrations;
pub mod persistence;

use crate::database::migrations::MIGRATIONS;
use deadpool_diesel::postgres::Pool;
use tracing::info;

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
