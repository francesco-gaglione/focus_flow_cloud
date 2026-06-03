use std::sync::Arc;

use chrono::NaiveDate;
use chrono::NaiveTime;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use domain::entities::tasks::task_priority::TaskPriority;
use domain::services::stats::completed_by_priority_service::CompletedByPriorityService;
use domain::services::stats::completed_focus_sessions_service::CompletedFocusSessionsService;
use domain::services::stats::completed_tasks_counts_service::CompletedTasksCountsService;
use domain::services::stats::count_by_category_service::CountByCategoryService;
use domain::services::stats::last_14d_counts_service::Last14dCountsService;
use domain::services::stats::last_8w_counts_service::Last8wCountsService;
use domain::services::stats::overdue_trend_service::OverdueTrendService;
use domain::services::stats::peak_window_service::{PeakWindowService, PeakWindowServiceError};
use domain::value_objects::stats::count_by_category::CategoryCount;
use domain::value_objects::stats::last_14d_counts::DayCounts;
use domain::value_objects::stats::last_8w_counts::WeekCounts;
use domain::value_objects::stats::overdue_trend::{OverdueTrend, OverdueTrendType};
use domain::value_objects::stats::peak_window::PwTimeRange;

use crate::repository_traits::category_persistence::CategoryPersistence;
use crate::repository_traits::focus_session_repository::{
    FindByFiltersCommand, FocusSessionRepository,
};
use crate::repository_traits::persistence_error::PersistenceError;
use crate::repository_traits::task_persistence::TaskPersistence;

#[derive(Debug, Error)]
pub enum GetStatsError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("Stats calculation error: {0}")]
    CalculationError(String),
}

impl From<PeakWindowServiceError> for GetStatsError {
    fn from(e: PeakWindowServiceError) -> Self {
        Self::CalculationError(e.to_string())
    }
}

pub type GetStatsResult<T> = Result<T, GetStatsError>;

#[derive(Debug)]
pub struct GetStatsCommand {
    pub user_id: Uuid,
    pub tz_offset_minutes: i32,
}

pub struct CompletedTasksCountsOutput {
    pub completed_today: i64,
    pub completed_this_week: i64,
    pub week_delta: i64,
    pub completed_this_month: i64,
    pub day_avg: f64,
    pub focus_sessions: i64,
}

pub struct PeakWindowRangeOutput {
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub count: usize,
}

impl From<&PwTimeRange> for PeakWindowRangeOutput {
    fn from(r: &PwTimeRange) -> Self {
        Self {
            start: r.start(),
            end: r.end(),
            count: r.count(),
        }
    }
}

pub struct CompletedByPriorityOutput {
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub urgent: usize,
}

pub struct CompletedFocusSessionsOutput {
    pub count: usize,
    pub avg_duration_secs: i64,
}

pub enum OverdueTrendTypeOutput {
    Increasing,
    Decreasing,
    Stable,
}

pub struct OverdueTrendOutput {
    pub trend_type: OverdueTrendTypeOutput,
    pub trend_value: f64,
}

impl From<OverdueTrend> for OverdueTrendOutput {
    fn from(t: OverdueTrend) -> Self {
        Self {
            trend_type: match t.trend_type {
                OverdueTrendType::Increasing => OverdueTrendTypeOutput::Increasing,
                OverdueTrendType::Decreasing => OverdueTrendTypeOutput::Decreasing,
                OverdueTrendType::Stable => OverdueTrendTypeOutput::Stable,
            },
            trend_value: t.trend_value,
        }
    }
}

pub struct CategoryCountOutput {
    pub category_id: Uuid,
    pub category_name: String,
    pub count: u64,
}

impl From<&CategoryCount> for CategoryCountOutput {
    fn from(c: &CategoryCount) -> Self {
        Self {
            category_id: c.category().id(),
            category_name: c.category().name().to_string(),
            count: c.count(),
        }
    }
}

pub struct DayCountOutput {
    pub day: NaiveDate,
    pub count: usize,
}

impl From<&DayCounts> for DayCountOutput {
    fn from(d: &DayCounts) -> Self {
        Self {
            day: d.day(),
            count: d.count(),
        }
    }
}

pub struct WeekCountOutput {
    pub week_start: NaiveDate,
    pub count: usize,
}

impl From<&WeekCounts> for WeekCountOutput {
    fn from(w: &WeekCounts) -> Self {
        Self {
            week_start: w.week_start(),
            count: w.count(),
        }
    }
}

pub struct StatsOutput {
    pub completed_tasks_counts: CompletedTasksCountsOutput,
    pub peak_window: Vec<PeakWindowRangeOutput>,
    pub completed_by_priority: CompletedByPriorityOutput,
    pub completed_focus_sessions: CompletedFocusSessionsOutput,
    pub overdue_trend: OverdueTrendOutput,
    pub count_by_category: Vec<CategoryCountOutput>,
    pub last_14d: Vec<DayCountOutput>,
    pub last_8w: Vec<WeekCountOutput>,
}

pub struct GetStatsUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
    focus_session_repository: Arc<dyn FocusSessionRepository>,
    category_persistence: Arc<dyn CategoryPersistence>,
}

impl GetStatsUseCase {
    pub fn new(
        task_persistence: Arc<dyn TaskPersistence>,
        focus_session_repository: Arc<dyn FocusSessionRepository>,
        category_persistence: Arc<dyn CategoryPersistence>,
    ) -> Self {
        Self {
            task_persistence,
            focus_session_repository,
            category_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: GetStatsCommand) -> GetStatsResult<StatsOutput> {
        let tasks = self.task_persistence.find_all().await?;
        let sessions = self
            .focus_session_repository
            .find_by_filters(FindByFiltersCommand {
                user_id: command.user_id,
                start_date: None,
                end_date: None,
                task_ids: None,
                session_type: None,
                min_concentration_score: None,
                max_concentration_score: None,
                has_notes: None,
            })
            .await?;
        let categories = self.category_persistence.find_all().await?;

        let completed_tasks_counts = CompletedTasksCountsService::calculate(&tasks, &sessions);
        let peak_window = PeakWindowService::new().calculate(&tasks, command.tz_offset_minutes)?;
        let completed_by_priority = CompletedByPriorityService::calculate(&tasks);
        let completed_focus_sessions = CompletedFocusSessionsService::calculate(&sessions);
        let overdue_trend = OverdueTrendService::calculate(&tasks);
        let count_by_category = CountByCategoryService::calculate(&tasks, &categories);
        let last_14d = Last14dCountsService::calculate(&tasks);
        let last_8w = Last8wCountsService::calculate(&tasks);

        Ok(StatsOutput {
            completed_tasks_counts: CompletedTasksCountsOutput {
                completed_today: completed_tasks_counts.completed_tasks(),
                completed_this_week: completed_tasks_counts.week_completed_tasks(),
                week_delta: completed_tasks_counts.current_week_delta(),
                completed_this_month: completed_tasks_counts.month_completed_tasks(),
                day_avg: completed_tasks_counts.day_avg(),
                focus_sessions: completed_tasks_counts.focus_sessions(),
            },
            peak_window: peak_window
                .time_ranges()
                .iter()
                .map(PeakWindowRangeOutput::from)
                .collect(),
            completed_by_priority: CompletedByPriorityOutput {
                low: completed_by_priority.get_count(TaskPriority::Low),
                medium: completed_by_priority.get_count(TaskPriority::Medium),
                high: completed_by_priority.get_count(TaskPriority::High),
                urgent: completed_by_priority.get_count(TaskPriority::Urgent),
            },
            completed_focus_sessions: CompletedFocusSessionsOutput {
                count: completed_focus_sessions.count(),
                avg_duration_secs: completed_focus_sessions.avg_duration().num_seconds(),
            },
            overdue_trend: overdue_trend.into(),
            count_by_category: count_by_category
                .counts()
                .iter()
                .map(CategoryCountOutput::from)
                .collect(),
            last_14d: last_14d.counts().iter().map(DayCountOutput::from).collect(),
            last_8w: last_8w.counts().iter().map(WeekCountOutput::from).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use mockall::predicate::eq;

    use crate::repository_traits::{
        category_persistence::MockCategoryPersistence,
        focus_session_repository::MockFocusSessionRepository,
        task_persistence::MockTaskPersistence,
    };

    use super::*;

    #[tokio::test]
    async fn test_get_stats_success() {
        let mut mock_tasks = MockTaskPersistence::new();
        mock_tasks.expect_find_all().returning(|| Ok(vec![]));

        let user_id = Uuid::new_v4();
        let mut mock_sessions = MockFocusSessionRepository::new();
        mock_sessions
            .expect_find_by_filters()
            .returning(|_| Ok(vec![]));

        let mut mock_categories = MockCategoryPersistence::new();
        mock_categories.expect_find_all().returning(|| Ok(vec![]));

        let use_case = GetStatsUseCase::new(
            Arc::new(mock_tasks),
            Arc::new(mock_sessions),
            Arc::new(mock_categories),
        );

        let result = use_case
            .execute(GetStatsCommand {
                user_id,
                tz_offset_minutes: 0,
            })
            .await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.completed_tasks_counts.completed_today, 0);
        assert_eq!(stats.peak_window.len(), 9);
        assert_eq!(stats.count_by_category.len(), 0);
        assert_eq!(stats.last_14d.len(), 14);
        assert_eq!(stats.last_8w.len(), 8);
    }

    #[tokio::test]
    async fn test_get_stats_persistence_error() {
        let mut mock_tasks = MockTaskPersistence::new();
        mock_tasks
            .expect_find_all()
            .returning(|| Err(PersistenceError::Unexpected("db down".to_string())));

        let mock_sessions = MockFocusSessionRepository::new();
        let mock_categories = MockCategoryPersistence::new();

        let use_case = GetStatsUseCase::new(
            Arc::new(mock_tasks),
            Arc::new(mock_sessions),
            Arc::new(mock_categories),
        );

        let result = use_case
            .execute(GetStatsCommand {
                user_id: Uuid::new_v4(),
                tz_offset_minutes: 0,
            })
            .await;
        assert!(matches!(result, Err(GetStatsError::PersistenceError(_))));
    }
}
