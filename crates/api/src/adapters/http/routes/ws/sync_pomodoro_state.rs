use crate::adapters::http::{
    app_state::AppState,
    dto::ws_msg::update_pomodoro_state::{
        UpdateCurrentSession, UpdatePomodoroState, UpdateWorkContext,
    },
};

pub async fn sync_pomodoro_state(state: &AppState) -> Result<UpdatePomodoroState, String> {
    let session_state = state.pomodoro_state.write().await;

    let current_session = session_state.current_session().map(|s| {
        UpdateCurrentSession::new(
            s.session_type().clone(),
            s.start_date(),
            s.category_id().cloned(),
            s.task_id().cloned(),
            s.note(),
            s.concentration_score(),
        )
    });

    let current_work_context = session_state.current_work_context();

    let update_work_context = UpdateWorkContext::new(
        current_work_context.category_id().cloned(),
        current_work_context.task_id().cloned(),
    );

    Ok(UpdatePomodoroState::new(
        update_work_context,
        current_session,
    ))
}
