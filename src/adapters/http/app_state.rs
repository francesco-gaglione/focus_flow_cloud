use crate::adapters::http::pomodoro_state::PomodoroState;
use crate::application::use_cases::user_settings::get_settings::GetSettingsUseCase;
use crate::application::use_cases::user_settings::update_setting::UpdateSettingUseCase;
use crate::application::use_cases::{
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
use crate::infra::config::AppConfig;
use axum::extract::ws::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;

pub type Clients = Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>;

#[derive(Clone)]
pub struct AppState {
    pub ws_clients: Clients,
    pub pomodoro_state: Arc<RwLock<PomodoroState>>,
    pub config: AppConfig,

    // Category Use Cases
    pub create_category_usecase: Arc<CreateCategoryUseCases>,
    pub delete_categories_usecase: Arc<DeleteCategoriesUseCases>,
    pub delete_category_usecase: Arc<DeleteCategoryUseCases>,
    pub get_category_and_task_usecase: Arc<GetCategoryAndTaskUseCases>,
    pub get_category_usecase: Arc<GetCategoryUseCases>,
    pub update_category_usecase: Arc<UpdateCategoryUseCases>,

    // Task Use Cases
    pub create_task_usecase: Arc<CreateTaskUseCase>,
    pub delete_tasks_usecase: Arc<DeleteTasksUseCase>,
    pub orphan_tasks_usecase: Arc<OrphanTasksUseCase>,
    pub update_task_usecase: Arc<UpdateTaskUseCase>,

    // Focus Session Use Cases
    pub create_manual_session_usecase: Arc<CreateManualSessionUseCase>,
    pub create_session_usecase: Arc<CreateSessionUseCase>,
    pub find_sessions_by_filters_usecase: Arc<FindSessionsByFiltersUseCase>,

    // Stats Use Cases
    pub calculate_stats_by_period_usecase: Arc<CalculateStatsByPeriodUseCase>,

    // User Setting Use Cases
    pub update_user_setting_usecase: Arc<UpdateSettingUseCase>,
    pub get_user_settings_usecase: Arc<GetSettingsUseCase>,
}
