use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

use crate::adapters::http::{
    dto::ws_msg::{update_workspace_ws::UpdateWorkspace, ws_message::WsErrorResponse},
    focus_sessions_state::FocusSessionsState,
};

pub async fn update_workspace(
    message: &UpdateWorkspace,
    sessions_state: &Arc<RwLock<FocusSessionsState>>,
) -> Result<UpdateWorkspace, String> {
    debug!("Starting session");

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
