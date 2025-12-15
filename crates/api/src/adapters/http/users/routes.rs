use axum::routing::post;
use axum::Router;

use crate::adapters::http::app_state::AppState;
use crate::adapters::http::users::create_user::create_user_api;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create_user_api))
}
