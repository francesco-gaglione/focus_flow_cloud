use crate::http::app_state::AppState;
use crate::http::task::create_task::create_task_api;
use crate::http::task::delete_tasks::delete_tasks_api;
use crate::http::task::get_tasks::get_tasks_api;
use crate::http::task::orphan_tasks::fetch_orphan_tasks_api;
use crate::http::task::update_task::update_task_api;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_task_api))
        .route("/", delete(delete_tasks_api))
        .route("/", get(get_tasks_api))
        .route("/{id}", put(update_task_api))
        .route("/orphans", get(fetch_orphan_tasks_api))
}
