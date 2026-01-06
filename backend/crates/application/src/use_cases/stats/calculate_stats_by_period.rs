use std::{collections::HashSet, sync::Arc};

use chrono::{DateTime, Utc};
use futures_util::future::join_all;
use uuid::Uuid;

use crate::{
    app_error::AppResult, use_cases::stats::command::calculate_stats_by_period::StatsPeriod,
};
use domain::entities::focus_session::SessionFilter;
use domain::entities::stats::Stats;
use domain::traits::{
    category_persistence::CategoryPersistence, focus_session_persistence::FocusSessionPersistence,
    task_persistence::TaskPersistence,
};

pub struct CalculateStatsByPeriodUseCase {
    category_persistence: Arc<dyn CategoryPersistence>,
    task_persistence: Arc<dyn TaskPersistence>,
    focus_session_persistence: Arc<dyn FocusSessionPersistence>,
}

impl CalculateStatsByPeriodUseCase {
    pub fn new(
        category_persistence: Arc<dyn CategoryPersistence>,
        task_persistence: Arc<dyn TaskPersistence>,
        focus_session_persistence: Arc<dyn FocusSessionPersistence>,
    ) -> Self {
        Self {
            category_persistence,
            task_persistence,
            focus_session_persistence,
        }
    }

    pub async fn execute(&self, period: StatsPeriod) -> AppResult<Stats> {
        let start_date: Option<DateTime<Utc>> = DateTime::from_timestamp(period.start_date, 0);
        let end_date: Option<DateTime<Utc>> = period
            .end_date
            .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0));

        let sessions = self
            .focus_session_persistence
            .find_by_filters(SessionFilter {
                user_id: period.user_id,
                start_date,
                end_date,
                category_ids: None,
                session_type: None,
                min_concentration_score: None,
                max_concentration_score: None,
                has_notes: None,
            })
            .await?;

        // Collect all unique IDs
        let mut category_ids: HashSet<Uuid> =
            sessions.iter().filter_map(|s| s.category_id()).collect();

        let task_ids: HashSet<Uuid> = sessions.iter().filter_map(|s| s.task_id()).collect();

        // Fetch Tasks (concurrently)
        let task_futures: Vec<_> = task_ids
            .iter()
            .map(|id| self.task_persistence.find_by_id(*id))
            .collect();

        let tasks_results = join_all(task_futures).await;
        let tasks: Vec<_> = tasks_results.into_iter().filter_map(|r| r.ok()).collect();

        // Add Category IDs from Tasks to the set, to ensure we have names for them
        for task in &tasks {
            if let Some(cat_id) = task.category_id() {
                category_ids.insert(cat_id);
            }
        }

        // Fetch Categories (concurrently)
        let category_futures: Vec<_> = category_ids
            .iter()
            .map(|id| self.category_persistence.find_by_id(*id))
            .collect();

        let categories_results = join_all(category_futures).await;
        let categories: Vec<_> = categories_results
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();

        // Determine if multi-day
        let is_multi_day = match (start_date, end_date) {
            (Some(start), Some(end)) => {
                let duration = end.signed_duration_since(start);
                duration.num_days() > 1
            }
            _ => false,
        };

        // Calculate stats using domain logic
        Ok(Stats::calculate(
            &sessions,
            &categories,
            &tasks,
            is_multi_day,
        )?)
    }
}
