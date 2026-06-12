use application::tasks::use_cases::category::create_category_usecase::CreateCategoryUseCases;
use application::tasks::use_cases::category::delete_categories_usecase::DeleteCategoriesUseCases;
use application::tasks::use_cases::category::get_all_category_usecase::GetAllCategoryUseCases;
use application::tasks::use_cases::category::update_category_usecase::UpdateCategoryUseCases;
use application::tasks::use_cases::focus_session::create_manual_session::CreateManualSessionUseCase;
use application::tasks::use_cases::focus_session::find_sessions_by_filters::FindSessionsByFiltersUseCase;
use application::tasks::use_cases::focus_session::update_focus_session::UpdateFocusSessionUseCase;
use application::tasks::use_cases::pomodoro_state::fetch_user_pomodoro_state::FetchUserPomodoroStateUseCase;
use application::tasks::use_cases::pomodoro_state::init_pomodoro_state::InitPomodoroStateUseCase;
use application::tasks::use_cases::pomodoro_state::pause_session::PauseSessionUseCase;
use application::tasks::use_cases::pomodoro_state::start_session::StartSessionUseCase;
use application::tasks::use_cases::pomodoro_state::terminate_session::TerminateSessionUseCase;
use application::tasks::use_cases::pomodoro_state::update_current_session::UpdateSessionUseCase;
use application::tasks::use_cases::pomodoro_state::update_pomodoro_context::UpdatePomodoroContextUseCase;
use application::tasks::use_cases::stats::get_stats::GetStatsUseCase;
use application::tasks::use_cases::task::add_subtask::AddSubTaskUseCase;
use application::tasks::use_cases::task::create_task::CreateTaskUseCase;
use application::tasks::use_cases::task::delete_task::DeleteTaskUseCase;
use application::tasks::use_cases::task::get_tasks::GetTasksUseCase;
use application::tasks::use_cases::task::update_subtask::UpdateSubTaskUseCase;
use application::tasks::use_cases::task::update_task::UpdateTaskUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct TasksState {
    // Category
    pub create_category_uc: Arc<CreateCategoryUseCases>,
    pub delete_categories_uc: Arc<DeleteCategoriesUseCases>,
    pub get_all_category_uc: Arc<GetAllCategoryUseCases>,
    pub update_category_uc: Arc<UpdateCategoryUseCases>,

    // Pomodoro
    pub init_pomodoro_state_uc: Arc<InitPomodoroStateUseCase>,
    pub pause_pomo_session_uc: Arc<PauseSessionUseCase>,
    pub fetch_pomo_session_uc: Arc<FetchUserPomodoroStateUseCase>,
    pub update_pomodoro_context_uc: Arc<UpdatePomodoroContextUseCase>,
    pub start_session_uc: Arc<StartSessionUseCase>,
    pub terminate_session_uc: Arc<TerminateSessionUseCase>,
    pub update_current_session_uc: Arc<UpdateSessionUseCase>,

    // Task
    pub create_task_uc: Arc<CreateTaskUseCase>,
    pub delete_tasks_uc: Arc<DeleteTaskUseCase>,
    pub get_tasks_uc: Arc<GetTasksUseCase>,
    pub update_task_uc: Arc<UpdateTaskUseCase>,
    pub update_subtask_uc: Arc<UpdateSubTaskUseCase>,
    pub add_subtask_uc: Arc<AddSubTaskUseCase>,

    // Stats
    pub get_stats_uc: Arc<GetStatsUseCase>,

    // Focus Session
    pub create_manual_session_uc: Arc<CreateManualSessionUseCase>,
    pub update_focus_session_uc: Arc<UpdateFocusSessionUseCase>,
    pub find_sessions_by_filters_uc: Arc<FindSessionsByFiltersUseCase>,
}
