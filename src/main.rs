use crate::repository::category_repository::CategoryRepository;
use crate::services::category_service::CategoryService;
use axum::Router;
use deadpool_diesel::postgres::Pool;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::sync::Arc;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod db_models;
mod dto;
mod entities;
mod repository;
mod schema;
mod services;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone)]
pub struct AppState {
    pub category_service: Arc<CategoryService>,
}

impl AppState {
    pub fn new(db_pool: Pool) -> Self {
        let category_repository = CategoryRepository::new(db_pool.clone());

        Self {
            category_service: Arc::new(CategoryService::new(category_repository)),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    let state: AppState = AppState::new(pool);

    // build our application with a single route
    let app = Router::new()
        .nest("/category", api::category_api::router())
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
