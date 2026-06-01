use crate::http::app_state::AppState;
use crate::http::push_subscription::handler::save_push_subscription_api;
use axum::routing::post;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(save_push_subscription_api))
}
