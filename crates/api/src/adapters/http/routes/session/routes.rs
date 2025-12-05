use crate::adapters::http::app_state::AppState;
use crate::adapters::http::routes::session::create_manual_session_api::create_manual_session_api;
use crate::adapters::http::routes::session::get_sessions::get_sessions;
use crate::adapters::http::routes::session::update_session_api::update_session_api;
use axum::routing::{get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_sessions))
        .route("/{id}", put(update_session_api))
        .route("/manual", post(create_manual_session_api))
}
