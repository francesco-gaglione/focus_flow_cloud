use crate::adapters::persistence::PostgresPersistence;
use crate::infra::database::run_migrations;
use deadpool_diesel::postgres::Pool;

pub async fn postgres_persistence(db_url: &str) -> PostgresPersistence {
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);

    let pool = Pool::builder(manager)
        .build()
        .expect("Failed to create database pool");

    run_migrations(&pool).await;

    PostgresPersistence::new(pool)
}
