use serde::{Deserialize, Serialize};

use crate::adapters::http::{
    dto::common::session_type_enum::SessionTypeEnum,
    pomodoro_state::{FocusSessionState, PomodoroState, WorkContext},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePomodoroState {
    current_session: Option<UpdateCurrentSession>,
    work_context: UpdateWorkContext,
}

impl UpdatePomodoroState {
    pub fn new(
        work_context: UpdateWorkContext,
        current_session: Option<UpdateCurrentSession>,
    ) -> Self {
        UpdatePomodoroState {
            current_session,
            work_context,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkContext {
    category_id: Option<String>,
    task_id: Option<String>,
}

impl UpdateWorkContext {
    pub fn new(category_id: Option<String>, task_id: Option<String>) -> Self {
        UpdateWorkContext {
            category_id,
            task_id,
        }
    }
}

impl From<PomodoroState> for UpdatePomodoroState {
    fn from(value: PomodoroState) -> Self {
        Self {
            current_session: value
                .current_session()
                .cloned()
                .map(|session| session.into()),
            work_context: value.current_work_context().into(),
        }
    }
}

impl From<FocusSessionState> for UpdateCurrentSession {
    fn from(value: FocusSessionState) -> Self {
        UpdateCurrentSession {
            session_type: value.session_type().clone(),
            session_start_time: value.start_date(),
            category_id: value.category_id().cloned(),
            task_id: value.task_id().cloned(),
            note: value.note(),
            concentration_score: value.concentration_score(),
        }
    }
}

impl From<WorkContext> for UpdateWorkContext {
    fn from(value: WorkContext) -> Self {
        Self {
            category_id: value.category_id().cloned(),
            task_id: value.task_id().cloned(),
        }
    }
}
