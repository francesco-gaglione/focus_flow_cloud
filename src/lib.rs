pub mod api;
pub mod config;
pub mod database;
pub mod db_models;
pub mod dto;
pub mod entities;
pub mod repository;
pub mod schema;
pub mod services;

use crate::config::{app_state::AppState, openapi::ApiDoc};
use axum::Router;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run() {
    dotenvy::dotenv().ok();

    init_tracing();

    let pool = database::setup_database().await;
    database::run_migrations(&pool).await;

    let state = AppState::new(pool);

    let app = build_router(state);

    start_server(app).await;
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api", api::routes::api_routes())
        .with_state(state)
}

async fn start_server(app: Router) {
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", server_port))
        .await
        .expect("Failed to bind server");

    log::info!("Running server on port {}", server_port);
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
