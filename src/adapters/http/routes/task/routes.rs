use crate::adapters::http::app_state::AppState;
use crate::adapters::http::routes::task::create_task_api::create_task_api;
use crate::adapters::http::routes::task::delete_tasks_api::delete_tasks_api;
use crate::adapters::http::routes::task::update_task_api::update_task_api;
use axum::Router;
use axum::routing::{delete, post, put};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/createTask", post(create_task_api))
        .route("/deleteTasks", delete(delete_tasks_api))
        .route("/updateTask", put(update_task_api))
}
