use crate::adapters::http::app_state::AppState;
use crate::adapters::http::pomodoro_state::PomodoroState;
use crate::application::use_cases::category_use_cases::CategoryUseCases;
use crate::application::use_cases::focus_session_use_cases::FocusSessionUseCases;
use crate::application::use_cases::stats_use_cases::StatsUseCases;
use crate::application::use_cases::task_use_cases::TaskUseCases;
use crate::infra::config::AppConfig;
use crate::infra::database::persistence::postgres_persistence;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use tracing_tree::HierarchicalLayer;

pub async fn init_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();

    let postgres_arc = Arc::new(postgres_persistence().await);

    let category_use_cases = CategoryUseCases::new(postgres_arc.clone(), postgres_arc.clone());
    let task_use_cases = TaskUseCases::new(postgres_arc.clone());
    let focus_session_use_cases = FocusSessionUseCases::new(postgres_arc.clone());
    let stats_use_cases = StatsUseCases::new(
        postgres_arc.clone(),
        postgres_arc.clone(),
        postgres_arc.clone(),
    );

    Ok(AppState {
        ws_clients: Arc::new(RwLock::new(HashMap::new())),
        pomodoro_state: Arc::new(RwLock::new(PomodoroState::default())),
        config,
        category_use_cases: Arc::new(category_use_cases),
        task_use_cases: Arc::new(task_use_cases),
        focus_session_use_cases: Arc::new(focus_session_use_cases),
        stats_use_cases: Arc::new(stats_use_cases),
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "focus_flow_cloud=info,tower_http=info".into());

    let tree_layer = HierarchicalLayer::new(2)
        .with_targets(true)
        .with_bracketed_fields(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(tree_layer)
        .try_init()
        .ok();
}
