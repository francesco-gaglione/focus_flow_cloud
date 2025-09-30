use crate::api::task::create_task_api::create_task_api;
use crate::config::app_state::AppState;
use axum::Router;
use axum::routing::post;

pub fn router() -> Router<AppState> {
    Router::new().route("/createTask", post(create_task_api))
}
