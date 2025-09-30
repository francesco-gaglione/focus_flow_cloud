use axum::Router;
use axum::routing::post;
use crate::api::category::create_category_api::create_category_api;
use crate::config::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/createCategory", post(create_category_api))
}
