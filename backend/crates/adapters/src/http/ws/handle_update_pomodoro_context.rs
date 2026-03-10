use crate::http::{
    app_state::AppState,
    ws::{error::WsHandlerResult, update_pomodoro_state::UpdatePomodoroState},
};
use application::use_cases::pomodoro_state::{
    fetch_user_pomodoro_state::FetchUserPomodoroStateCommand,
    update_pomodoro_context::UpdatePomodoroContextCommand,
};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePomodoroContext {
    category_id: Option<String>,
    task_id: Option<String>,
}

impl UpdatePomodoroContext {
    pub fn new(category_id: Option<String>, task_id: Option<String>) -> Self {
        Self {
            category_id,
            task_id,
        }
    }

    pub fn category_id(&self) -> Option<String> {
        self.category_id.clone()
    }

    pub fn task_id(&self) -> Option<String> {
        self.task_id.clone()
    }
}

pub async fn handle_update_pomodoro_context(
    message: &UpdatePomodoroContext,
    state: &AppState,
    user_id: Uuid,
) -> WsHandlerResult<UpdatePomodoroState> {
    debug!("Updating pomodoro context for user {}", user_id);

    let command = UpdatePomodoroContextCommand {
        user_id,
        category_id: message.category_id().map(|id| id.parse().unwrap()),
        task_id: message.task_id().map(|id| id.parse().unwrap()),
    };

    state.update_pomodoro_context_uc.execute(command).await?;

    let pomodoro_state = state
        .fetch_pomo_session_uc
        .execute(FetchUserPomodoroStateCommand { user_id })
        .await?;

    Ok(pomodoro_state.into())
}
