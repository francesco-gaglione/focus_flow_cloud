use application::tasks::use_cases::stats::get_stats::{
    GetStatsCommand, GetStatsError, OverdueTrendTypeOutput, StatsOutput,
};
use axum::extract::{Extension, Query, State};
use axum::Json;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct GetStatsResponseDto {
    pub completed_tasks_counts: CompletedTasksCountsDto,
    pub peak_window: Vec<PeakWindowRangeDto>,
    pub completed_by_priority: CompletedByPriorityDto,
    pub completed_focus_sessions: CompletedFocusSessionsDto,
    pub overdue_trend: OverdueTrendDto,
    pub count_by_category: Vec<CategoryCountDto>,
    pub last_14d: Vec<DayCountDto>,
    pub last_8w: Vec<WeekCountDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CompletedTasksCountsDto {
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub completed_today: i64,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub completed_this_week: i64,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub week_delta: i64,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub completed_this_month: i64,
    pub day_avg: f64,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub focus_sessions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PeakWindowRangeDto {
    pub start: String,
    pub end: String,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CompletedByPriorityDto {
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub low: usize,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub medium: usize,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub high: usize,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub urgent: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CompletedFocusSessionsDto {
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub count: usize,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub avg_duration_secs: i64,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OverdueTrendTypeDto {
    Increasing,
    Decreasing,
    #[default]
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct OverdueTrendDto {
    pub trend_type: OverdueTrendTypeDto,
    pub trend_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CategoryCountDto {
    #[cfg_attr(feature = "ts", ts(type = "string"))]
    pub category_id: Uuid,
    pub category_name: String,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct DayCountDto {
    #[cfg_attr(feature = "ts", ts(type = "string"))]
    pub day: NaiveDate,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct WeekCountDto {
    #[cfg_attr(feature = "ts", ts(type = "string"))]
    pub week_start: NaiveDate,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub count: usize,
}

use crate::http_error::{map_persistence_error, HttpError, HttpResult};
use crate::openapi::STATS_TAG;
use crate::shared::http::app_state::AppState;
use crate::shared::http::model::session_model::UserSession;

impl From<GetStatsError> for HttpError {
    fn from(err: GetStatsError) -> Self {
        match err {
            GetStatsError::PersistenceError(e) => map_persistence_error(e),
            GetStatsError::CalculationError(msg) => HttpError::GenericError(msg),
        }
    }
}

fn to_response_dto(s: StatsOutput) -> GetStatsResponseDto {
    GetStatsResponseDto {
        completed_tasks_counts: CompletedTasksCountsDto {
            completed_today: s.completed_tasks_counts.completed_today,
            completed_this_week: s.completed_tasks_counts.completed_this_week,
            week_delta: s.completed_tasks_counts.week_delta,
            completed_this_month: s.completed_tasks_counts.completed_this_month,
            day_avg: s.completed_tasks_counts.day_avg,
            focus_sessions: s.completed_tasks_counts.focus_sessions,
        },
        peak_window: s
            .peak_window
            .iter()
            .map(|r| PeakWindowRangeDto {
                start: r.start.format("%H:%M:%S").to_string(),
                end: r.end.format("%H:%M:%S").to_string(),
                count: r.count,
            })
            .collect(),
        completed_by_priority: CompletedByPriorityDto {
            low: s.completed_by_priority.low,
            medium: s.completed_by_priority.medium,
            high: s.completed_by_priority.high,
            urgent: s.completed_by_priority.urgent,
        },
        completed_focus_sessions: CompletedFocusSessionsDto {
            count: s.completed_focus_sessions.count,
            avg_duration_secs: s.completed_focus_sessions.avg_duration_secs,
        },
        overdue_trend: OverdueTrendDto {
            trend_type: match s.overdue_trend.trend_type {
                OverdueTrendTypeOutput::Increasing => OverdueTrendTypeDto::Increasing,
                OverdueTrendTypeOutput::Decreasing => OverdueTrendTypeDto::Decreasing,
                OverdueTrendTypeOutput::Stable => OverdueTrendTypeDto::Stable,
            },
            trend_value: s.overdue_trend.trend_value,
        },
        count_by_category: s
            .count_by_category
            .iter()
            .map(|c| CategoryCountDto {
                category_id: c.category_id,
                category_name: c.category_name.clone(),
                count: c.count,
            })
            .collect(),
        last_14d: s
            .last_14d
            .iter()
            .map(|d| DayCountDto {
                day: d.day,
                count: d.count,
            })
            .collect(),
        last_8w: s
            .last_8w
            .iter()
            .map(|w| WeekCountDto {
                week_start: w.week_start,
                count: w.count,
            })
            .collect(),
    }
}

#[derive(Deserialize)]
pub struct GetStatsQuery {
    tz_offset: Option<i32>,
}

#[utoipa::path(
    get,
    path = "/api/stats",
    tag = STATS_TAG,
    summary = "Get all stats",
    responses(
        (status = 200, description = "Stats fetched successfully", body = GetStatsResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]

pub async fn get_stats_api(
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    Query(query): Query<GetStatsQuery>,
) -> HttpResult<Json<GetStatsResponseDto>> {
    let result = state
        .tasks
        .get_stats_uc
        .execute(GetStatsCommand {
            user_id: session.user_id,
            tz_offset_minutes: query.tz_offset.unwrap_or(0),
        })
        .await?;

    Ok(Json(to_response_dto(result)))
}
