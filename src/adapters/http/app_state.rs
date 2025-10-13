use crate::adapters::http::focus_sessions_state::FocusSessionsState;
use crate::application::use_cases::category_use_cases::CategoryUseCases;
use crate::application::use_cases::focus_session_use_cases::FocusSessionUseCases;
use crate::application::use_cases::task_use_cases::TaskUseCases;
use crate::infra::config::AppConfig;
use axum::extract::FromRef;
use axum::extract::ws::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc::UnboundedSender;

pub type Clients = Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>;

#[derive(Clone)]
pub struct AppState {
    pub ws_clients: Clients,
    pub focus_session_state: Arc<RwLock<FocusSessionsState>>,
    pub config: AppConfig,
    pub category_use_cases: Arc<CategoryUseCases>,
    pub task_use_cases: Arc<TaskUseCases>,
    pub focus_session_use_cases: Arc<FocusSessionUseCases>,
}

impl FromRef<AppState> for Arc<CategoryUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.category_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<TaskUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.task_use_cases.clone()
    }
}
