use axum::{routing::any, Router};

use crate::adapters::http::{app_state::AppState, routes::ws::ws_handler::session_handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/test", any(session_handler))
}