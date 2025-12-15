use crate::adapters::http::app_state::AppState;
use axum::Router;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest(
            "/category",
            crate::adapters::http::category::routes::router(),
        )
        .nest("/task", crate::adapters::http::task::routes::router())
        .nest("/session", crate::adapters::http::session::routes::router())
        .nest(
            "/setting",
            crate::adapters::http::user_setting::routes::router(),
        )
        .nest("/users", crate::adapters::http::users::routes::router())
        .nest("/auth", crate::adapters::http::auth::routes::routes())
        .nest("/stats", crate::adapters::http::stats::routes::router())
}

pub fn ws_routes() -> Router<AppState> {
    Router::new().nest("/ws", crate::adapters::http::ws::routes::router())
}
