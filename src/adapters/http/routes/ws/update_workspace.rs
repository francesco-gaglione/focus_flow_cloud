use tracing::debug;

use crate::adapters::http::{
    app_state::AppState,
    dto::{
        common::session_type_enum::SessionTypeEnum, ws_msg::update_workspace_ws::UpdateWorkspace,
    },
};

pub async fn update_workspace(
    message: &UpdateWorkspace,
    state: &AppState,
) -> Result<UpdateWorkspace, String> {
    debug!("Updating workspace");
    let sessions_state = state.focus_session_state.clone();
    let mut state = sessions_state.write().await;

    if let Some(ref current_session) = state.current_session {
        if current_session.session_type == SessionTypeEnum::Work {
            return Err("Cannot change workspace if session is running".to_string());
        }
    }

    state.workspace.category_id = message.category_id.clone();
    state.workspace.task_id = message.task_id.clone();

    Ok(UpdateWorkspace {
        category_id: state.workspace.category_id.clone(),
        task_id: state.workspace.task_id.clone(),
    })
}
