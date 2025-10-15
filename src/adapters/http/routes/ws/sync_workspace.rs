use crate::adapters::http::{
    app_state::AppState,
    dto::ws_msg::sync_workspace_ws::{SyncSession, SyncWorkspace},
};

pub async fn sync_workspace(state: &AppState) -> Result<SyncWorkspace, String> {
    let session_state = state.focus_session_state.write().await;

    let session = session_state.current_session.clone().map(|s| SyncSession {
        session_type: s.session_type,
        start_date: s.start_date,
        note: s.note,
    });

    Ok(SyncWorkspace {
        session,
        category_id: session_state.workspace.category_id.clone(),
        task_id: session_state.workspace.task_id.clone(),
    })
}
