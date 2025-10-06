pub mod category;
pub mod session;
pub mod task;

use axum::Router;
use crate::adapters::http::app_state::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/category", category::routes::router())
        .nest("/task", task::routes::router())
}
