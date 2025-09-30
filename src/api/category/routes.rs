use crate::api::category::create_category_api::create_category_api;
use crate::api::category::get_categories::get_categories_and_tasks_api;
use crate::config::app_state::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/createCategory", post(create_category_api))
        .route("/getCategoriesAndTasks", get(get_categories_and_tasks_api))
}
