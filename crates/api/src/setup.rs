use crate::adapters::http::app_state::AppState;
use crate::adapters::http::pomodoro_state::PomodoroState;
use application::use_cases::focus_session::update_focus_session::UpdateFocusSessionUseCase;
use application::use_cases::task::get_tasks::GetTasksUseCase;
use application::use_cases::user_settings::get_settings::GetSettingsUseCase;
use application::use_cases::user_settings::update_setting::UpdateSettingUseCase;
use application::use_cases::{
    category::{
        create_category_usecase::CreateCategoryUseCases,
        delete_categories_usecase::DeleteCategoriesUseCases,
        delete_category_usecase::DeleteCategoryUseCases,
        get_category_and_task_usecase::GetCategoryAndTaskUseCases,
        get_category_usecase::GetCategoryUseCases, update_category_usecase::UpdateCategoryUseCases,
    },
    focus_session::{
        create_manual_session::CreateManualSessionUseCase, create_session::CreateSessionUseCase,
        find_sessions_by_filters::FindSessionsByFiltersUseCase,
    },
    stats::calculate_stats_by_period::CalculateStatsByPeriodUseCase,
    task::{
        create_task::CreateTaskUseCase, delete_tasks::DeleteTasksUseCase,
        orphan_tasks::OrphanTasksUseCase, update_task::UpdateTaskUseCase,
    },
};
use infrastructure::config::AppConfig;
use infrastructure::database::persistence::postgres_persistence;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use tracing_tree::HierarchicalLayer;

pub async fn init_app_state(config: AppConfig) -> Result<AppState, Box<dyn std::error::Error>> {
    let postgres_arc = Arc::new(postgres_persistence(&config.database_url).await);

    // Category Use Cases
    let create_category_usecase = Arc::new(CreateCategoryUseCases::new(postgres_arc.clone()));
    let delete_categories_usecase = Arc::new(DeleteCategoriesUseCases::new(postgres_arc.clone()));
    let delete_category_usecase = Arc::new(DeleteCategoryUseCases::new(postgres_arc.clone()));
    let get_category_and_task_usecase = Arc::new(GetCategoryAndTaskUseCases::new(
        postgres_arc.clone(),
        postgres_arc.clone(),
    ));
    let get_category_usecase = Arc::new(GetCategoryUseCases::new(postgres_arc.clone()));
    let update_category_usecase = Arc::new(UpdateCategoryUseCases::new(postgres_arc.clone()));

    // Task Use Cases
    let create_task_usecase = Arc::new(CreateTaskUseCase::new(postgres_arc.clone()));
    let get_tasks_usecase = Arc::new(GetTasksUseCase::new(postgres_arc.clone()));
    let delete_tasks_usecase = Arc::new(DeleteTasksUseCase::new(postgres_arc.clone()));
    let orphan_tasks_usecase = Arc::new(OrphanTasksUseCase::new(postgres_arc.clone()));
    let update_task_usecase = Arc::new(UpdateTaskUseCase::new(postgres_arc.clone()));

    // Focus Session Use Cases
    let create_manual_session_usecase =
        Arc::new(CreateManualSessionUseCase::new(postgres_arc.clone()));
    let create_session_usecase = Arc::new(CreateSessionUseCase::new(postgres_arc.clone()));
    let update_focus_session_usecase =
        Arc::new(UpdateFocusSessionUseCase::new(postgres_arc.clone()));
    let find_sessions_by_filters_usecase =
        Arc::new(FindSessionsByFiltersUseCase::new(postgres_arc.clone()));

    // Stats Use Cases
    let calculate_stats_by_period_usecase = Arc::new(CalculateStatsByPeriodUseCase::new(
        postgres_arc.clone(),
        postgres_arc.clone(),
        postgres_arc.clone(),
    ));

    // User Setting Use Cases
    let get_user_settings_usecase = Arc::new(GetSettingsUseCase::new(postgres_arc.clone()));
    let update_user_setting_usecase = Arc::new(UpdateSettingUseCase::new(postgres_arc.clone()));

    Ok(AppState {
        ws_clients: Arc::new(RwLock::new(HashMap::new())),
        pomodoro_state: Arc::new(RwLock::new(PomodoroState::default())),
        config,
        create_category_usecase,
        delete_categories_usecase,
        delete_category_usecase,
        get_category_and_task_usecase,
        get_category_usecase,
        update_category_usecase,
        get_tasks_usecase,
        create_task_usecase,
        delete_tasks_usecase,
        orphan_tasks_usecase,
        update_task_usecase,
        create_manual_session_usecase,
        update_focus_session_usecase,
        create_session_usecase,
        find_sessions_by_filters_usecase,
        calculate_stats_by_period_usecase,
        update_user_setting_usecase,
        get_user_settings_usecase,
    })
}

pub fn init_tracing() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "api=info,tower_http=info".into());

    let tree_layer = HierarchicalLayer::new(2)
        .with_targets(true)
        .with_bracketed_fields(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(tree_layer)
        .try_init()
        .ok();
}
