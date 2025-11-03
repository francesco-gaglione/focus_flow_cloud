use crate::adapters::http::app_state::AppState;
use crate::adapters::http::routes::session::create_manual_session_api::create_manual_session_api;
use crate::adapters::http::routes::session::get_sessions::get_sessions;
use axum::routing::{get, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_sessions))
        .route("/manual", post(create_manual_session_api))
}
