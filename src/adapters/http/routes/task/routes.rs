use crate::adapters::http::app_state::AppState;
use crate::adapters::http::routes::task::create_task_api::create_task_api;
use crate::adapters::http::routes::task::delete_tasks_api::delete_tasks_api;
use crate::adapters::http::routes::task::get_tasks_api::get_tasks_api;
use crate::adapters::http::routes::task::orphan_tasks_api::fetch_orphan_tasks_api;
use crate::adapters::http::routes::task::update_task_api::update_task_api;
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
