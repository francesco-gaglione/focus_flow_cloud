use crate::config::AppConfig;
use crate::http::pomodoro_state::PomodoroState;
use application::use_cases::focus_session::update_focus_session::UpdateFocusSessionUseCase;
use application::use_cases::task::complete_task::CompleteTaskUseCase;
use application::use_cases::task::get_scheduled_tasks::GetScheduledTasksUseCase;
use application::use_cases::task::get_tasks::GetTasksUseCase;
use application::use_cases::user::get_user_info::GetUserInfoUseCase;
use application::use_cases::user::login_user::LoginUseCase;
use application::use_cases::user::refresh_token::RefreshTokenUseCase;
use application::use_cases::user::register_user::RegisterUserUseCase;
use application::use_cases::user::update_password::UpdateUserPasswordUseCase;
use application::use_cases::user::update_user_username::UpdateUserUsernameUseCase;
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
use axum::extract::ws::Message;
use domain::services::token_service::TokenService;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use uuid::Uuid;

pub type Clients = Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>;

#[derive(Clone)]
pub struct AppState {
    pub ws_clients: Clients,
    pub pomodoro_states: Arc<RwLock<HashMap<Uuid, Arc<RwLock<PomodoroState>>>>>,
    pub config: AppConfig,

    // Category Use Cases
    pub create_category_uc: Arc<CreateCategoryUseCases>,
    pub delete_categories_uc: Arc<DeleteCategoriesUseCases>,
    pub delete_category_uc: Arc<DeleteCategoryUseCases>,
    pub get_category_and_task_uc: Arc<GetCategoryAndTaskUseCases>,
    pub get_category_uc: Arc<GetCategoryUseCases>,
    pub update_category_uc: Arc<UpdateCategoryUseCases>,

    // Task Use Cases
    pub create_task_uc: Arc<CreateTaskUseCase>,
    pub delete_tasks_uc: Arc<DeleteTasksUseCase>,
    pub orphan_tasks_uc: Arc<OrphanTasksUseCase>,
    pub get_tasks_uc: Arc<GetTasksUseCase>,
    pub update_task_uc: Arc<UpdateTaskUseCase>,
    pub complete_task_uc: Arc<CompleteTaskUseCase>,
    pub get_scheduled_task_uc: Arc<GetScheduledTasksUseCase>,

    // Focus Session Use Cases
    pub create_manual_session_uc: Arc<CreateManualSessionUseCase>,
    pub update_focus_session_uc: Arc<UpdateFocusSessionUseCase>,
    pub create_session_uc: Arc<CreateSessionUseCase>,
    pub find_sessions_by_filters_uc: Arc<FindSessionsByFiltersUseCase>,

    // Stats Use Cases
    pub calculate_stats_by_period_uc: Arc<CalculateStatsByPeriodUseCase>,

    // User Setting Use Cases
    pub update_user_setting_uc: Arc<UpdateSettingUseCase>,
    pub get_user_settings_uc: Arc<GetSettingsUseCase>,

    // User Use Cases
    pub register_user_uc: Arc<RegisterUserUseCase>,
    pub login_uc: Arc<LoginUseCase>,
    pub refresh_token_uc: Arc<RefreshTokenUseCase>,
    pub update_password_uc: Arc<UpdateUserPasswordUseCase>,
    pub update_user_username_uc: Arc<UpdateUserUsernameUseCase>,
    pub get_user_info_uc: Arc<GetUserInfoUseCase>,

    // Services
    pub token_service: Arc<dyn TokenService>,

    // Version
    pub version: String,
}
