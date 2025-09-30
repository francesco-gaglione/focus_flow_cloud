use crate::api;
use crate::config::app_state::AppState;
use axum::Router;

pub fn api_routes() -> Router<AppState> {
    Router::new().nest("/category", api::category::routes::router())
}
