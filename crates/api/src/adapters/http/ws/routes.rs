use axum::{routing::any, Router};

use crate::adapters::http::app_state::AppState;
use crate::adapters::http::ws::ws_handler::session_handler;

pub fn router() -> Router<AppState> {
    Router::new().route("/session", any(session_handler))
}
