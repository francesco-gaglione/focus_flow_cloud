use crate::adapters::http::app_state::AppState;
use crate::application::use_cases::category_use_cases::CategoryUseCases;
use crate::application::use_cases::focus_session_use_cases::FocusSessionUseCases;
use crate::application::use_cases::task_use_cases::TaskUseCases;
use crate::infra::config::AppConfig;
use crate::infra::database::persistence::postgres_persistence;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

pub async fn init_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();

    let postgres_arc = Arc::new(postgres_persistence().await);

    let category_use_cases = CategoryUseCases::new(postgres_arc.clone(), postgres_arc.clone());
    let task_use_cases = TaskUseCases::new(postgres_arc.clone());
    let focus_session_use_cases = FocusSessionUseCases::new(postgres_arc.clone());

    Ok(AppState {
        ws_clients: Arc::new(RwLock::new(HashMap::new())),
        config,
        category_use_cases: Arc::new(category_use_cases),
        task_use_cases: Arc::new(task_use_cases),
        focus_session_use_cases: Arc::new(focus_session_use_cases),
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "axum_trainer=debug,tower_http=debug".into());

    // Console (pretty logs)
    let console_layer = fmt::layer()
        .with_target(false) // don’t show target (module path)
        .with_level(true) // show log level
        .pretty(); // human-friendly, with colors

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .try_init()
        .ok();
}
