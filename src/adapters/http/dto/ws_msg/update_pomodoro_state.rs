use serde::{Deserialize, Serialize};

use crate::adapters::http::dto::common::session_type_enum::SessionTypeEnum;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePomodoroState {
    current_session: UpdateCurrentSession,
    work_context: UpdateWorkContext,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkContext {
    category_id: Option<String>,
    task_id: Option<String>,
}
