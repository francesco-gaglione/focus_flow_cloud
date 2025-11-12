use tracing::debug;

use crate::adapters::http::{
    app_state::AppState, dto::ws_msg::update_workspace_ws::UpdateWorkspace,
};

pub async fn update_workspace(
    message: &UpdateWorkspace,
    state: &AppState,
) -> Result<UpdateWorkspace, String> {
    debug!("Updating workspace");
    let pomodoro_state = state.pomodoro_state.clone();
    let mut state = pomodoro_state.write().await;

    if let Some(ref current_session) = state.current_session() {
        if current_session.is_work_session() {
            return Err("Cannot change workspace if session is running".to_string());
        }
    }

    state.update_work_context(message.category_id.clone(), message.task_id.clone());

    Ok(UpdateWorkspace {
        category_id: state.workspace.category_id.clone(),
        task_id: state.workspace.task_id.clone(),
    })
}
