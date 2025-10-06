use crate::adapters::http::app_state::AppState;
use crate::adapters::http::routes::category::create_category_api::create_category_api;
use crate::adapters::http::routes::category::delete_categories_api::delete_categories_api;
use crate::adapters::http::routes::category::get_categories_and_tasks_api::get_categories_and_tasks_api;
use axum::Router;
use axum::routing::{delete, get, post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/createCategory", post(create_category_api))
        .route("/getCategoriesAndTasks", get(get_categories_and_tasks_api))
        .route("/deleteCategories", delete(delete_categories_api))
}
