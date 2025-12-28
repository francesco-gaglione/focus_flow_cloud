use crate::http::app_state::AppState;
use crate::http::auth::login::login_api;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().route("/login", post(login_api))
}
