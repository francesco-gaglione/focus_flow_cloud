use crate::shared::http::app_state::AppState;
use crate::user::http::auth::auth_middleware::auth_middleware;
use axum::Router;

pub fn api_routes(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new().nest("/auth", crate::user::http::auth::routes::routes());
    let version_routes =
        Router::new().nest("/version", crate::shared::http::version::routes::router());

    let protected_routes = Router::new()
        .merge(crate::flashcards::http::flashcards::routes::routes())
        .merge(crate::flashcards::http::folder::routes::routes())
        .nest("/category", crate::tasks::http::category::routes::router())
        .nest("/task", crate::tasks::http::task::routes::router())
        .nest(
            "/focus-session",
            crate::tasks::http::session::routes::router(),
        )
        .nest(
            "/setting",
            crate::user::http::user_setting::routes::router(),
        )
        .nest("/users", crate::user::http::users::routes::router())
        .nest("/stats", crate::tasks::http::stats::routes::router())
        .nest(
            "/push-subscriptions",
            crate::shared::http::push_subscription::routes::router(),
        )
        .nest(
            "/reminders",
            crate::shared::http::reminder::routes::router(),
        )
        .layer(axum::middleware::from_fn_with_state(state, auth_middleware));

    Router::new()
        .merge(auth_routes)
        .merge(version_routes)
        .merge(protected_routes)
}

pub fn ws_routes(state: AppState) -> Router<AppState> {
    crate::tasks::http::ws::routes::router()
        .layer(axum::middleware::from_fn_with_state(state, auth_middleware))
}
