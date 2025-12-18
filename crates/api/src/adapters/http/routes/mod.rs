use crate::adapters::http::{app_state::AppState, auth::auth_middleware::auth_middleware};
use axum::Router;

pub fn api_routes(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new().nest("/auth", crate::adapters::http::auth::routes::routes());

    let protected_routes = Router::new()
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
        .nest("/stats", crate::adapters::http::stats::routes::router())
        .layer(axum::middleware::from_fn_with_state(state, auth_middleware));

    Router::new().merge(auth_routes).merge(protected_routes)
}

pub fn ws_routes() -> Router<AppState> {
    Router::new().nest("/ws", crate::adapters::http::ws::routes::router())
}
