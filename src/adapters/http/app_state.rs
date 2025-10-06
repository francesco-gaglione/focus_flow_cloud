use crate::application::use_cases::category_use_cases::CategoryUseCases;
use crate::application::use_cases::task_use_cases::TaskUseCases;
use crate::infra::config::AppConfig;
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub category_use_cases: Arc<CategoryUseCases>,
    pub task_use_cases: Arc<TaskUseCases>,
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
