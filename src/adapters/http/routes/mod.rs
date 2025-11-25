pub mod category;
pub mod session;
pub mod stats;
pub mod task;
pub mod user_setting;
pub mod ws;

use crate::adapters::http::app_state::AppState;
use axum::Router;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/categories", category::routes::router())
        .nest("/tasks", task::routes::router())
        .nest("/focus-sessions", session::routes::router())
        .nest("/stats", stats::routes::router())
        .nest("/user-settings", user_setting::routes::router())
}

pub fn ws_routes() -> Router<AppState> {
    Router::new().nest("/workspace", ws::routes::router())
}
