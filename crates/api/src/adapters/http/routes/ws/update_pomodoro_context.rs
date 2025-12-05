use tracing::debug;

use crate::adapters::http::{
    app_state::AppState,
    dto::ws_msg::{
        update_pomodoro_context::UpdatePomodoroContext, update_pomodoro_state::UpdatePomodoroState,
    },
};

pub async fn update_pomodoro_context(
    message: &UpdatePomodoroContext,
    state: &AppState,
) -> Result<UpdatePomodoroState, String> {
    debug!("Updating pomodoro context");
    let pomodoro_state = state.pomodoro_state.clone();
    let mut state = pomodoro_state.write().await;

    if let Some(current_session) = state.current_session() {
        if current_session.is_work_session() {
            return Err("Cannot change workspace if session is running".to_string());
        }
    }

    state.update_work_context(message.category_id(), message.task_id());

    Ok(UpdatePomodoroState::from(state.clone()))
}
