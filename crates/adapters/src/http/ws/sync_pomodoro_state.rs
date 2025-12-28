use crate::http::{
    app_state::AppState,
    ws::update_pomodoro_state::{UpdateCurrentSession, UpdatePomodoroState, UpdateWorkContext},
};
use uuid::Uuid;

pub async fn sync_pomodoro_state(
    state: &AppState,
    user_id: Uuid,
) -> Result<UpdatePomodoroState, String> {
    let states_map = state.pomodoro_states.read().await;
    let user_state = states_map
        .get(&user_id)
        .ok_or("User state not found".to_string())?
        .clone();
    drop(states_map);

    let session_state = user_state.read().await;

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
