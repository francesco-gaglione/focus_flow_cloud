use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::HttpResult;
use crate::openapi::STATS_TAG;
use application::use_cases::stats::command::calculate_stats_by_period::StatsPeriod;
use axum::extract::{Extension, Query, State};
use axum::Json;
use domain::entities::stats::{
    CategoryDistributionItem, ConcentrationPeriod, DailyActivityDistributionItem,
    DailyActivityItem, Stats, TaskDistributionItem,
};
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
    pub most_concentrated_period: ConcentrationPeriodDto,
    pub less_concentrated_period: ConcentrationPeriodDto,
    pub concentration_distribution: [u32; 5],
    pub category_distribution: Vec<CategoryDistributionDto>,
    pub task_distribution: Vec<TaskDistributionDto>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskDistributionDto {
    pub category_name: Option<String>,
    pub category_id: Option<String>,
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

impl From<Stats> for GetStatsByPeriodResponseDto {
    fn from(stats: Stats) -> Self {
        Self {
            total_sessions: stats.total_sessions(),
            total_breaks: stats.total_breaks(),
            total_focus_time: stats.total_focus_time(),
            total_break_time: stats.total_break_time(),
            most_concentrated_period: stats.most_concentrated_period().clone().into(),
            less_concentrated_period: stats.less_concentrated_period().clone().into(),
            concentration_distribution: *stats.concentration_distribution(),
            category_distribution: stats
                .category_distribution()
                .to_vec()
                .into_iter()
                .map(|item| item.into())
                .collect(),
            task_distribution: stats
                .task_distribution()
                .to_vec()
                .into_iter()
                .map(|item| item.into())
                .collect(),
            daily_activity: stats
                .daily_activity()
                .to_vec()
                .into_iter()
                .map(|item| item.into())
                .collect(),
        }
    }
}

impl From<ConcentrationPeriod> for ConcentrationPeriodDto {
    fn from(period: ConcentrationPeriod) -> Self {
        match period {
            ConcentrationPeriod::Morning => ConcentrationPeriodDto::Morning,
            ConcentrationPeriod::Afternoon => ConcentrationPeriodDto::Afternoon,
        }
    }
}

impl From<CategoryDistributionItem> for CategoryDistributionDto {
    fn from(item: CategoryDistributionItem) -> Self {
        Self {
            category_name: item.category_name().to_string(),
            category_id: item.category_id().to_string(),
            total_focus_time: item.total_focus_time(),
            percentage: item.percentage(),
        }
    }
}

impl From<TaskDistributionItem> for TaskDistributionDto {
    fn from(item: TaskDistributionItem) -> Self {
        Self {
            category_name: item.category_name().map(|s| s.to_string()),
            category_id: item.category_id().map(|id| id.to_string()),
            task_name: item.task_name().to_string(),
            total_focus_time: item.total_focus_time(),
            percentage: item.percentage(),
        }
    }
}

impl From<DailyActivityItem> for DailyActivityDto {
    fn from(item: DailyActivityItem) -> Self {
        Self {
            date: item
                .date()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp(),
            category_distribution: item
                .category_distribution()
                .to_vec()
                .into_iter()
                .map(|dist| dist.into())
                .collect(),
        }
    }
}

impl From<DailyActivityDistributionItem> for DailyActivityDistributionDto {
    fn from(item: DailyActivityDistributionItem) -> Self {
        Self {
            category_name: item.category_name().to_string(),
            category_id: item.category_id().to_string(),
            total_focus_time: item.total_focus_time(),
        }
    }
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
    State(state): State<AppState>,
    Extension(session): Extension<UserSession>,
    query: Query<GetStatsByPeriodDto>,
) -> HttpResult<Json<GetStatsByPeriodResponseDto>> {
    debug!("query: {:?}", query);

    let stats_period = StatsPeriod {
        user_id: session.user_id,
        start_date: query.start_date,
        end_date: query.end_date,
    };

    let stats = state
        .calculate_stats_by_period_usecase
        .execute(stats_period)
        .await?;

    Ok(Json(stats.into()))
}
