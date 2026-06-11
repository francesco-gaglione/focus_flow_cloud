use axum::{routing::get, Router};

use crate::shared::http::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/pending",
        get(super::get_pending::get_pending_reminders_api),
    )
}
