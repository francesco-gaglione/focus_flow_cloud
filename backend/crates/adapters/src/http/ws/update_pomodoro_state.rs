use application::use_cases::pomodoro_state::fetch_user_pomodoro_state::{
    FetchUserPomodoroStateOutput, UserCurrentSession,
};
use serde::{Deserialize, Serialize};

use crate::http::dto::common::session_type_enum::SessionTypeEnum;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePomodoroState {
    current_session: Option<UpdateCurrentSession>,
    category_id: Option<String>,
    task_id: Option<String>,
}

impl UpdatePomodoroState {
    pub fn new(
        category_id: Option<String>,
        task_id: Option<String>,
        current_session: Option<UpdateCurrentSession>,
    ) -> Self {
        UpdatePomodoroState {
            current_session,
            category_id,
            task_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCurrentSession {
    session_type: SessionTypeEnum,
    session_start_time: i64,
    category_id: Option<String>,
    task_id: Option<String>,
    note: Option<String>,
    concentration_score: Option<i32>,
}

impl UpdateCurrentSession {
    pub fn new(
        session_type: SessionTypeEnum,
        session_start_time: i64,
        category_id: Option<String>,
        task_id: Option<String>,
        note: Option<String>,
        concentration_score: Option<i32>,
    ) -> Self {
        UpdateCurrentSession {
            session_type,
            session_start_time,
            category_id,
            task_id,
            note,
            concentration_score,
        }
    }
}

impl From<FetchUserPomodoroStateOutput> for UpdatePomodoroState {
    fn from(value: FetchUserPomodoroStateOutput) -> Self {
        Self {
            current_session: value.user_current_session.map(|s| s.into()),
            category_id: value.category_id,
            task_id: value.task_id,
        }
    }
}

impl From<UserCurrentSession> for UpdateCurrentSession {
    fn from(value: UserCurrentSession) -> Self {
        Self {
            session_type: value.session_type.into(),
            session_start_time: value.session_start_time,
            category_id: value.category_id,
            task_id: value.task_id,
            note: value.note,
            concentration_score: Some(value.concentration_score),
        }
    }
}
