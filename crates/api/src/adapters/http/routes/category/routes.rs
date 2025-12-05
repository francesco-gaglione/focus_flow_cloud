use crate::adapters::http::app_state::AppState;
use crate::adapters::http::routes::category::create_category_api::create_category_api;
use crate::adapters::http::routes::category::delete_categories_api::delete_categories_api;
use crate::adapters::http::routes::category::get_categories_and_tasks_api::get_categories_and_tasks_api;
use crate::adapters::http::routes::category::get_category::get_category;
use crate::adapters::http::routes::category::update_category_api::update_category_api;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_category_api))
        .route("/", get(get_categories_and_tasks_api))
        .route("/{id}", get(get_category))
        .route("/{id}", delete(delete_categories_api))
        .route("/{id}", put(update_category_api))
}
