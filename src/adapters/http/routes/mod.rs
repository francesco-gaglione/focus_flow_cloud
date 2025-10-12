pub mod category;
pub mod session;
pub mod task;
pub mod ws;

use crate::adapters::http::app_state::AppState;
use axum::Router;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/category", category::routes::router())
        .nest("/task", task::routes::router())
        .nest("/focusSession", session::routes::router())
}

pub fn ws_routes() -> Router<AppState> {
    Router::new().nest("/test", ws::routes::router())
}
