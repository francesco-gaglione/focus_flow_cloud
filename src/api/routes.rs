use crate::config::app_state::AppState;
use axum::Router;

pub fn api_routes() -> Router<AppState> {
    Router::new().nest("/category", super::category_api::router())
}
