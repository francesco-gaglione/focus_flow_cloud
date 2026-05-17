use crate::config::AppConfig;
use application::use_cases::category::get_all_category_usecase::GetAllCategoryUseCases;
use application::use_cases::focus_session::update_focus_session::UpdateFocusSessionUseCase;
use application::use_cases::pomodoro_state::fetch_user_pomodoro_state::FetchUserPomodoroStateUseCase;
use application::use_cases::pomodoro_state::init_pomodoro_state::InitPomodoroStateUseCase;
use application::use_cases::pomodoro_state::pause_session::PauseSessionUseCase;
use application::use_cases::pomodoro_state::start_session::StartSessionUseCase;
use application::use_cases::pomodoro_state::terminate_session::TerminateSessionUseCase;
use application::use_cases::pomodoro_state::update_current_session::UpdateSessionUseCase;
use application::use_cases::pomodoro_state::update_pomodoro_context::UpdatePomodoroContextUseCase;
use application::use_cases::stats::get_stats::GetStatsUseCase;
use application::use_cases::task::add_subtask::AddSubTaskUseCase;
use application::use_cases::task::get_tasks::GetTasksUseCase;
use application::use_cases::task::update_subtask::UpdateSubTaskUseCase;
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
        update_category_usecase::UpdateCategoryUseCases,
    },
    focus_session::{
        create_manual_session::CreateManualSessionUseCase,
        find_sessions_by_filters::FindSessionsByFiltersUseCase,
    },
    task::{
        create_task::CreateTaskUseCase, delete_task::DeleteTaskUseCase,
        update_task::UpdateTaskUseCase,
    },
};
use axum::extract::ws::Message;
use domain::services::token_service::TokenService;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;

pub type Clients = Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>;

#[derive(Clone)]
pub struct AppState {
    pub ws_clients: Clients,
    pub config: AppConfig,

    // Category Use Cases
    pub create_category_uc: Arc<CreateCategoryUseCases>,
    pub delete_categories_uc: Arc<DeleteCategoriesUseCases>,
    pub get_all_category_uc: Arc<GetAllCategoryUseCases>,
    pub update_category_uc: Arc<UpdateCategoryUseCases>,

    // Pomodoro state use cases
    pub init_pomodoro_state_uc: Arc<InitPomodoroStateUseCase>,
    pub pause_pomo_session_uc: Arc<PauseSessionUseCase>,
    pub fetch_pomo_session_uc: Arc<FetchUserPomodoroStateUseCase>,
    pub update_pomodoro_context_uc: Arc<UpdatePomodoroContextUseCase>,
    pub start_session_uc: Arc<StartSessionUseCase>,
    pub terminate_session_uc: Arc<TerminateSessionUseCase>,
    pub update_current_session_uc: Arc<UpdateSessionUseCase>,

    // Task Use Cases
    pub create_task_uc: Arc<CreateTaskUseCase>,
    pub delete_tasks_uc: Arc<DeleteTaskUseCase>,
    pub get_tasks_uc: Arc<GetTasksUseCase>,
    pub update_task_uc: Arc<UpdateTaskUseCase>,
    pub update_subtask_uc: Arc<UpdateSubTaskUseCase>,
    pub add_subtask_uc: Arc<AddSubTaskUseCase>,

    // Stats Use Cases
    pub get_stats_uc: Arc<GetStatsUseCase>,

    // Focus Session Use Cases
    pub create_manual_session_uc: Arc<CreateManualSessionUseCase>,
    pub update_focus_session_uc: Arc<UpdateFocusSessionUseCase>,
    pub find_sessions_by_filters_uc: Arc<FindSessionsByFiltersUseCase>,

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
