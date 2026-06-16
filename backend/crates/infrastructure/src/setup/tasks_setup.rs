use adapters::shared::http::app_state::tasks_state::TasksState;
use adapters::shared::persistence::impls::reminder_worker_port_impl::ReminderWorkerPortImpl;
use adapters::shared::persistence::PostgresPersistence;
use adapters::tasks::persistence::impls::pomodoro_state_in_memory_impl::PomodoroStateInMermoryImpl;
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

pub fn build_tasks_state(
    postgres: Arc<PostgresPersistence>,
    pomodoro: Arc<PomodoroStateInMermoryImpl>,
    reminder_worker: Arc<ReminderWorkerPortImpl>,
) -> TasksState {
    TasksState {
        init_pomodoro_state_uc: Arc::new(InitPomodoroStateUseCase::new(pomodoro.clone())),
        pause_pomo_session_uc: Arc::new(PauseSessionUseCase::new(
            pomodoro.clone(),
            postgres.clone(),
        )),
        fetch_pomo_session_uc: Arc::new(FetchUserPomodoroStateUseCase::new(pomodoro.clone())),
        update_pomodoro_context_uc: Arc::new(UpdatePomodoroContextUseCase::new(pomodoro.clone())),
        start_session_uc: Arc::new(StartSessionUseCase::new(pomodoro.clone(), postgres.clone())),
        terminate_session_uc: Arc::new(TerminateSessionUseCase::new(
            pomodoro.clone(),
            postgres.clone(),
        )),
        update_current_session_uc: Arc::new(UpdateSessionUseCase::new(pomodoro.clone())),
        create_category_uc: Arc::new(CreateCategoryUseCases::new(postgres.clone())),
        delete_categories_uc: Arc::new(DeleteCategoriesUseCases::new(postgres.clone())),
        get_all_category_uc: Arc::new(GetAllCategoryUseCases::new(postgres.clone())),
        update_category_uc: Arc::new(UpdateCategoryUseCases::new(postgres.clone())),
        create_task_uc: Arc::new(CreateTaskUseCase::new(
            postgres.clone(),
            postgres.clone(),
            reminder_worker,
        )),
        delete_tasks_uc: Arc::new(DeleteTaskUseCase::new(postgres.clone())),
        get_tasks_uc: Arc::new(GetTasksUseCase::new(postgres.clone(), postgres.clone())),
        update_task_uc: Arc::new(UpdateTaskUseCase::new(postgres.clone())),
        update_subtask_uc: Arc::new(UpdateSubTaskUseCase::new(postgres.clone())),
        add_subtask_uc: Arc::new(AddSubTaskUseCase::new(postgres.clone())),
        get_stats_uc: Arc::new(GetStatsUseCase::new(
            postgres.clone(),
            postgres.clone(),
            postgres.clone(),
        )),
        create_manual_session_uc: Arc::new(CreateManualSessionUseCase::new(postgres.clone())),
        update_focus_session_uc: Arc::new(UpdateFocusSessionUseCase::new(postgres.clone())),
        find_sessions_by_filters_uc: Arc::new(FindSessionsByFiltersUseCase::new(postgres.clone())),
    }
}
