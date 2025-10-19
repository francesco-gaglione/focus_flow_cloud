use crate::adapters::http::app_state::AppState;
use crate::adapters::http::routes::stats::calculate_stats_by_period::calculate_stats_by_period_api;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router<AppState> {
    Router::new().route("/getStatsByPeriod", get(calculate_stats_by_period_api))
}
