use crate::http::app_state::AppState;
use crate::http::task::complete_subtask::complete_sub_task_api;
use crate::http::task::complete_task::complete_task_api;
use crate::http::task::create_task::create_task_api;
use crate::http::task::delete_tasks::delete_tasks_api;
use crate::http::task::get_tasks::get_tasks_api;
use crate::http::task::update_task::update_task_api;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_task_api))
        .route("/", delete(delete_tasks_api))
        .route("/", get(get_tasks_api))
        .route("/{id}", put(update_task_api))
        .route("/complete", post(complete_task_api))
        .route("/subtask/complete", post(complete_sub_task_api))
}
