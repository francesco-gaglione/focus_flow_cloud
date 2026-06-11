pub mod flashcard_setup;
pub mod shared_setup;
pub mod tasks_setup;
pub mod user_setup;

use adapters::config::AppConfig;
use adapters::shared::http::app_state::AppState;
use adapters::shared::persistence::impls::persistence::postgres_persistence;
use adapters::shared::persistence::impls::reminder_worker_port_impl::{
    spawn_reminder_worker, ReminderWorkerPortImpl,
};
use adapters::tasks::persistence::impls::pomodoro_state_in_memory_impl::PomodoroStateInMermoryImpl;
use application::user::traits::password_hasher::PasswordHasher;
use application::user::traits::user_persistence::UserPersistence;
use flashcard_setup::build_flashcard_state;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use shared_setup::build_shared_state;
use std::collections::HashMap;
use std::sync::Arc;
use tasks_setup::build_tasks_state;
use tokio::sync::RwLock;
use tracing::info;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use user_setup::build_user_state;

use crate::crypto::password_hasher::Argon2Hasher;
use crate::database::run_migrations;
use crate::services::jwt_service::JwtService;

pub async fn init_app_state(
    config: AppConfig,
    version: String,
) -> Result<AppState, Box<dyn std::error::Error>> {
    let persistence = postgres_persistence(&config.database_url).await;
    run_migrations(&persistence.pool).await;
    let postgres_arc = Arc::new(persistence);

    let sqlx_pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("Failed to create sqlx pool for apalis");
    let reminder_worker = Arc::new(ReminderWorkerPortImpl::new(sqlx_pool.clone()));

    info!("Spawning reminder worker");
    spawn_reminder_worker(
        &sqlx_pool,
        postgres_arc.clone(),
        postgres_arc.clone(),
        config.vapid_private_key.clone(),
    )
    .await;
    info!("Reminder worker spawned");

    let pomodoro_state_arc = Arc::new(PomodoroStateInMermoryImpl::new());
    let argon_hasher = Arc::new(Argon2Hasher::new());
    let password_policy =
        Arc::new(adapters::user::auth::password_policy_impl::PasswordPolicyImpl::new());
    let token_service = Arc::new(JwtService::new(config.jwt_secret.clone()));

    seed_admin_user(&postgres_arc, &argon_hasher, &config).await;

    Ok(AppState {
        ws_clients: Arc::new(RwLock::new(HashMap::new())),
        config: config.clone(),
        token_service: token_service.clone(),
        version,
        tasks: build_tasks_state(postgres_arc.clone(), pomodoro_state_arc, reminder_worker),
        user: build_user_state(
            postgres_arc.clone(),
            argon_hasher,
            password_policy,
            token_service,
        ),
        shared: build_shared_state(postgres_arc.clone()),
        flashcards: build_flashcard_state(postgres_arc.clone()),
    })
}

async fn seed_admin_user(
    postgres_arc: &Arc<adapters::shared::persistence::PostgresPersistence>,
    argon_hasher: &Arc<Argon2Hasher>,
    config: &AppConfig,
) {
    if let (Some(username), Some(password)) = (&config.admin_username, &config.admin_password) {
        use domain::user::entities::{user::User, user_role::UserRole};
        use tracing::{error, info};

        info!("Checking for admin user: {}", username);

        match postgres_arc.find_user_by_username(username).await {
            Ok(_) => {
                info!(
                    "Admin user '{}' already exists. Skipping creation.",
                    username
                );
            }
            Err(_) => {
                info!("Admin user '{}' not found. Creating...", username);
                match argon_hasher.hash_password(password) {
                    Ok(hashed_password) => {
                        let admin_user =
                            User::new(username.clone(), hashed_password, UserRole::Admin);
                        match postgres_arc.create_user(admin_user).await {
                            Ok(id) => info!(
                                "Successfully created admin user '{}' with ID: {}",
                                username, id
                            ),
                            Err(e) => error!("Failed to create admin user: {:?}", e),
                        }
                    }
                    Err(e) => error!("Failed to hash admin password: {:?}", e),
                }
            }
        }
    }
}

pub fn init_tracing<Sink>(sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "focus_flow_cloud=debug,api=debug,domain=debug,infrastructure=debug,application=debug,tower_http=info,axum=info,info".into()
    });

    let formatting_layer = BunyanFormattingLayer::new("focus_flow_cloud".to_string(), sink);

    let otel_layer = std::env::var("OTLP_ENDPOINT").ok().map(|endpoint| {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(endpoint)
            .build()
            .expect("Failed to build OTLP span exporter");

        let provider = opentelemetry_sdk::trace::TracerProvider::builder()
            .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
            .with_resource(Resource::new(vec![opentelemetry::KeyValue::new(
                "service.name",
                "focus_flow_cloud",
            )]))
            .build();

        global::set_tracer_provider(provider.clone());
        let tracer = provider.tracer("focus_flow_cloud");
        tracing_opentelemetry::layer().with_tracer(tracer)
    });

    LogTracer::init().expect("Failed to set logger");

    let registry = Registry::default()
        .with(filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(otel_layer);

    set_global_default(registry).expect("Failed to set subscriber");
}
