use crate::http::app_state::AppState;
use crate::http::users::create_user::create_user_api;
use crate::http::users::get_info::get_user_info_api;
use crate::http::users::update_password::update_password_api;
use crate::http::users::update_username::update_username_api;
use axum::routing::{get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user_api))
        .route("/password", put(update_password_api))
        .route("/username", put(update_username_api))
        .route("/me", get(get_user_info_api))
}
