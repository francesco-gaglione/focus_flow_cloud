use crate::shared::http::app_state::AppState;
use crate::tasks::http::stats::get_stats::get_stats_api;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_stats_api))
}
