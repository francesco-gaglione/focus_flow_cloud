use crate::http::{
    app_state::AppState,
    ws::{error::WsHandlerResult, update_pomodoro_state::UpdatePomodoroState},
};
use application::tasks::use_cases::pomodoro_state::{
    fetch_user_pomodoro_state::FetchUserPomodoroStateCommand,
    update_current_session::UpdateSessionCommand,
};
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoteUpdate {
    pub new_note: String,
}

pub async fn handle_note_update(
    message: &NoteUpdate,
    state: &AppState,
    user_id: Uuid,
) -> WsHandlerResult<UpdatePomodoroState> {
    debug!("Updating current session note for user {}", user_id);

    let command = UpdateSessionCommand {
        user_id,
        new_note: Some(message.new_note.clone()),
        new_concentration_score: None,
    };

    state.update_current_session_uc.execute(command).await?;

    let pomodoro_state = state
        .fetch_pomo_session_uc
        .execute(FetchUserPomodoroStateCommand { user_id })
        .await?;

    Ok(pomodoro_state.into())
}
