use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::HttpResult;
use crate::openapi::STATS_TAG;

use axum::extract::{Extension, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetStatsByPeriodDto {
    pub start_date: i64,
    pub end_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetStatsByPeriodResponseDto {
    pub total_sessions: usize,
    pub total_breaks: usize,
    pub total_focus_time: i64,
    pub total_break_time: i64,
    pub focus_pause_ratio: f32,
    pub most_concentrated_period: ConcentrationPeriodDto,
    pub less_concentrated_period: ConcentrationPeriodDto,
    pub concentration_distribution: [u32; 5],
    pub category_distribution: Vec<CategoryDistributionDto>,
    pub daily_activity: Vec<DailyActivityDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConcentrationPeriodDto {
    Morning,
    Afternoon,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDistributionDto {
    pub category_name: String,
    pub category_id: String,
    pub total_focus_time: i64,
    pub percentage: f32,
    pub task_distribution: Vec<TaskDistributionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskDistributionDto {
    pub task_name: String,
    pub total_focus_time: i64,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DailyActivityDto {
    pub date: i64,
    pub category_distribution: Vec<DailyActivityDistributionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DailyActivityDistributionDto {
    pub category_name: String,
    pub category_id: String,
    pub total_focus_time: i64,
}

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
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn calculate_stats_by_period_api(
    State(_state): State<AppState>,
    Extension(_session): Extension<UserSession>,
    query: Query<GetStatsByPeriodDto>,
) -> HttpResult<Json<GetStatsByPeriodResponseDto>> {
    debug!("query: {:?}", query);

    //Mock response
    let res = GetStatsByPeriodResponseDto {
        total_sessions: todo!(),
        total_breaks: todo!(),
        total_focus_time: todo!(),
        total_break_time: todo!(),
        focus_pause_ratio: todo!(),
        most_concentrated_period: todo!(),
        less_concentrated_period: todo!(),
        concentration_distribution: todo!(),
        category_distribution: todo!(),
        daily_activity: todo!(),
    };

    Ok(Json(res))
}
