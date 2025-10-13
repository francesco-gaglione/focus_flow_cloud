use axum::{Router, routing::any};

use crate::adapters::http::{app_state::AppState, routes::ws::ws_handler::session_handler};

pub fn router() -> Router<AppState> {
    Router::new().route("/session", any(session_handler))
}
