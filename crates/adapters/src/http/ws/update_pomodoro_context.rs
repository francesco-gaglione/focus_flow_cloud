use crate::http::{app_state::AppState, ws::update_pomodoro_state::UpdatePomodoroState};
use serde::{Deserialize, Serialize};
use tracing::debug;
use validator::Validate;
use uuid::Uuid;

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

pub async fn update_pomodoro_context(
    message: &UpdatePomodoroContext,
    state: &AppState,
    user_id: Uuid,
) -> Result<UpdatePomodoroState, String> {
    debug!("Updating pomodoro context for user {}", user_id);
    
    let states_map = state.pomodoro_states.read().await;
    let user_state = states_map
        .get(&user_id)
        .ok_or("User state not found".to_string())?
        .clone();
    drop(states_map);

    let mut session_state = user_state.write().await;

    if let Some(current_session) = session_state.current_session() {
        if current_session.is_work_session() {
            return Err("Cannot change workspace if session is running".to_string());
        }
    }

    session_state.update_work_context(message.category_id(), message.task_id());

    Ok(UpdatePomodoroState::from(session_state.clone()))
}
