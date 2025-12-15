
use crate::adapters::http::app_state::AppState;
use axum::Router;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/categories", crate::adapters::http::category::routes::router())
        .nest("/tasks", crate::adapters::http::task::routes::router())
        .nest("/focus-sessions", crate::adapters::http::session::routes::router())
        .nest("/stats", crate::adapters::http::stats::routes::router())
        .nest("/user-settings", crate::adapters::http::user_setting::routes::router())
        .nest("/users", crate::adapters::http::users::routes::router())
}

pub fn ws_routes() -> Router<AppState> {
    Router::new().nest("/ws", crate::adapters::http::ws::routes::router())
}
