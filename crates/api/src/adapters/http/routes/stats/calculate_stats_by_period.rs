use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::stats_api::{GetStatsByPeriodDto, GetStatsByPeriodResponseDto};
use crate::adapters::http_error::HttpResult;
use crate::adapters::openapi::STATS_TAG;
use application::use_cases::stats::command::calculate_stats_by_period::StatsPeriod;
use axum::extract::{Query, State};
use axum::Json;
use tracing::debug;

#[utoipa::path(
    get,
    path = "/api/stats/period",
    tag = STATS_TAG,
    summary = "Get stats by period",
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
) -> HttpResult<Json<GetStatsByPeriodResponseDto>> {
    debug!("query: {:?}", query);

    let stats_period = StatsPeriod {
        start_date: query.start_date,
        end_date: query.end_date,
    };

    let stats = state
        .calculate_stats_by_period_usecase
        .execute(stats_period)
        .await?;

    Ok(Json(stats.into()))
}
