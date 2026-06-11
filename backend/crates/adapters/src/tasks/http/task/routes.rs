use crate::shared::http::app_state::AppState;
use crate::tasks::http::task::create_subtask::create_subtask_api;
use crate::tasks::http::task::create_task::create_task_api;
use crate::tasks::http::task::delete_tasks::delete_tasks_api;
use crate::tasks::http::task::get_tasks::get_tasks_api;
use crate::tasks::http::task::update_subtask::update_subtask_api;
use crate::tasks::http::task::update_task::update_task_api;
use axum::routing::{delete, get, patch, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_task_api))
        .route("/", delete(delete_tasks_api))
        .route("/", get(get_tasks_api))
        .route("/{id}", patch(update_task_api))
        .route("/{id}/subtask", post(create_subtask_api))
        .route("/{id}/subtask/{subtask_id}", patch(update_subtask_api))
}
