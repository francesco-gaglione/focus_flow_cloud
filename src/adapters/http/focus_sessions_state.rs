use crate::adapters::http::dto::common::session_type_enum::SessionTypeEnum;

#[derive(Debug, Default)]
pub struct FocusSessionsState {
    pub consecute_sessions: Vec<FocusSessionState>,
    pub current_session: Option<FocusSessionState>,
    pub workspace: Workspace,
}

#[derive(Debug)]
pub struct FocusSessionState {
    pub session_type: SessionTypeEnum,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub category_id: Option<String>,
    pub task_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct Workspace {
    pub category_id: Option<String>,
    pub task_id: Option<String>,
}
