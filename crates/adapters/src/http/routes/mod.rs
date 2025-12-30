use crate::http::{app_state::AppState, auth::auth_middleware::auth_middleware};
use axum::Router;

pub fn api_routes(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new().nest("/auth", crate::http::auth::routes::routes());

    let protected_routes = Router::new()
        .nest("/category", crate::http::category::routes::router())
        .nest("/task", crate::http::task::routes::router())
        .nest("/focus-session", crate::http::session::routes::router())
        .nest("/setting", crate::http::user_setting::routes::router())
        .nest("/users", crate::http::users::routes::router())
        .nest("/stats", crate::http::stats::routes::router())
        .layer(axum::middleware::from_fn_with_state(state, auth_middleware));

    Router::new().merge(auth_routes).merge(protected_routes)
}

pub fn ws_routes(state: AppState) -> Router<AppState> {
    crate::http::ws::routes::router()
        .layer(axum::middleware::from_fn_with_state(state, auth_middleware))
}
