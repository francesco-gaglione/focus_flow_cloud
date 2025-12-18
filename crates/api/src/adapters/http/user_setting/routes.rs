use axum::routing::{get, patch};
use axum::Router;

use crate::adapters::http::app_state::AppState;
use crate::adapters::http::user_setting::get_user_settings::get_settings_api;
use crate::adapters::http::user_setting::update_setting::update_setting_api;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", patch(update_setting_api))
        .route("/", get(get_settings_api))
}
