use crate::adapters::http::app_state::AppState;
use crate::adapters::http::routes::session::create_manual_session_api::create_manual_session_api;
use axum::Router;
use axum::routing::{delete, get, post};

pub fn router() -> Router<AppState> {
    Router::new().route("/createManualSession", post(create_manual_session_api))
}
