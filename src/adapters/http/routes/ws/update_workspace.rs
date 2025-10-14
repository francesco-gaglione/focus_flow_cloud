use tracing::debug;

use crate::adapters::http::{
    app_state::AppState, dto::ws_msg::update_workspace_ws::UpdateWorkspace,
};

pub async fn update_workspace(
    message: &UpdateWorkspace,
    state: &AppState,
) -> Result<UpdateWorkspace, String> {
    debug!("Starting session");
    let sessions_state = state.focus_session_state.clone();

    let mut state = sessions_state.write().await;

    if state.current_session.is_some() {
        return Err("Cannot change workspace if session is running".to_string());
    }

    state.workspace.category_id = message.category_id.clone();
    state.workspace.task_id = message.task_id.clone();

    Ok(UpdateWorkspace {
        category_id: state.workspace.category_id.clone(),
        task_id: state.workspace.task_id.clone(),
    })
}
