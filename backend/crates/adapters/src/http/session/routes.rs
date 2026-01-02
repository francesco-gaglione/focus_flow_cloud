use crate::http::app_state::AppState;
use crate::http::session::create_manual_session::create_manual_session_api;
use crate::http::session::get_sessions::get_sessions;
use crate::http::session::update_session::update_session_api;
use axum::routing::{get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_sessions))
        .route("/{id}", put(update_session_api))
        .route("/manual", post(create_manual_session_api))
}
