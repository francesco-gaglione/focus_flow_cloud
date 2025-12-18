use crate::adapters::http::app_state::AppState;
use crate::adapters::http::stats::calculate_stats_by_period::calculate_stats_by_period_api;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().route("/period", get(calculate_stats_by_period_api))
}
