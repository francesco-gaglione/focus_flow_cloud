use crate::api::session::create_manual_session_api::create_manual_session_api;
use crate::config::app_state::AppState;
use axum::Router;
use axum::routing::{delete, get, post};

pub fn router() -> Router<AppState> {
    Router::new().route("/createManualSession", post(create_manual_session_api))
}
