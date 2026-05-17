use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CompletedTasksCountsDto {
    pub completed_today: i64,
    pub completed_this_week: i64,
    pub week_delta: i64,
    pub completed_this_month: i64,
    pub day_avg: f64,
    pub focus_sessions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct PeakWindowRangeDto {
    #[cfg_attr(feature = "openapi", schema(value_type = String, example = "08:00:00"))]
    pub start: String,
    #[cfg_attr(feature = "openapi", schema(value_type = String, example = "10:00:00"))]
    pub end: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CompletedByPriorityDto {
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub urgent: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CompletedFocusSessionsDto {
    pub count: usize,
    pub avg_duration_secs: i64,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OverdueTrendTypeDto {
    Increasing,
    Decreasing,
    #[default]
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct OverdueTrendDto {
    pub trend_type: OverdueTrendTypeDto,
    pub trend_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CategoryCountDto {
    pub category_id: Uuid,
    pub category_name: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct DayCountDto {
    #[cfg_attr(feature = "openapi", schema(value_type = String, example = "2025-01-15"))]
    pub day: NaiveDate,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct WeekCountDto {
    #[cfg_attr(feature = "openapi", schema(value_type = String, example = "2025-01-13"))]
    pub week_start: NaiveDate,
    pub count: usize,
}
