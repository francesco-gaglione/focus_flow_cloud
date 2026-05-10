use crate::http::app_state::AppState;
use crate::http::category::create_category::create_category_api;
use crate::http::category::delete_categories::delete_categories_api;
use crate::http::category::get_all_categories::get_all_categories_api;
use crate::http::category::update_category::update_category_api;
use axum::routing::{delete, get, patch, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_category_api))
        .route("/categories", get(get_all_categories_api))
        .route("/{id}", delete(delete_categories_api))
        .route("/{id}", patch(update_category_api))
}
