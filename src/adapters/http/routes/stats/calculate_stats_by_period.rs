use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::stats_api::{GetStatsByPeriodDto, GetStatsByPeriodResponseDto};
use crate::adapters::openapi::STATS_TAG;
use crate::application::app_error::AppResult;
use crate::application::use_cases::commands::calculate_stats_by_period::StatsPeriod;
use axum::extract::{Query, State};
use axum::Json;
use tracing::debug;

#[utoipa::path(
    get,
    path = "/api/stats/period",
    tag = STATS_TAG,
    params(
        GetStatsByPeriodDto
    ),
    responses(
        (status = 200, description = "Stats fetched successfully", body = GetStatsByPeriodResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn calculate_stats_by_period_api(
    State(state): State<AppState>,
    query: Query<GetStatsByPeriodDto>,
) -> AppResult<Json<GetStatsByPeriodResponseDto>> {
    debug!("query: {:?}", query);

    let stats_period = StatsPeriod {
        start_date: query.start_date,
        end_date: query.end_date,
    };

    let stats = state
        .stats_use_cases
        .calculate_stats_by_period(stats_period)
        .await?;

    Ok(Json(stats.into()))
}
