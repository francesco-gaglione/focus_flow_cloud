use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, NaiveDate, Timelike, Utc};
use futures_util::future::join_all;
use uuid::Uuid;

use crate::{
    application::{
        app_error::AppResult,
        traits::{CategoryPersistence, FocusSessionPersistence, TaskPersistence},
        use_cases::{
            commands::calculate_stats_by_period::StatsPeriod,
            persistance_command::find_session_by_filters_data::FindSessionByFiltersData,
        },
    },
    domain::entities::{
        focus_session::FocusSession,
        focus_session_type::FocusSessionType,
        stats::{
            CategoryDistributionItem, ConcentrationPeriod, DailyActivityDistributionItem,
            DailyActivityItem, Stats, TaskDistributionItem,
        },
    },
};

#[derive(Clone)]
pub struct StatsUseCases {
    category_persistence: Arc<dyn CategoryPersistence>,
    task_persistence: Arc<dyn TaskPersistence>,
    focus_session_persistence: Arc<dyn FocusSessionPersistence>,
}

impl StatsUseCases {
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

    pub async fn calculate_stats_by_period(&self, period: StatsPeriod) -> AppResult<Stats> {
        let start_date: Option<DateTime<Utc>> = DateTime::from_timestamp(period.start_date, 0);
        let end_date: Option<DateTime<Utc>> = period
            .end_date
            .and_then(|timestamp| DateTime::from_timestamp(timestamp, 0));

        let sessions = self
            .focus_session_persistence
            .find_by_filters(FindSessionByFiltersData {
                start_date,
                end_date,
                category_ids: None,
                session_type: None,
                min_concentration_score: None,
                max_concentration_score: None,
            })
            .await?;

        let is_multi_day = Self::is_multi_day_period(start_date, end_date);

        let (work_sessions, break_sessions): (Vec<_>, Vec<_>) = sessions
            .iter()
            .partition(|s| s.session_type() == FocusSessionType::Work);

        let total_sessions = work_sessions.len();
        let total_breaks = break_sessions.len();

        let total_focus_time: i64 = work_sessions
            .iter()
            .filter_map(|s| s.actual_duration())
            .sum();

        let total_break_time: i64 = break_sessions
            .iter()
            .filter_map(|s| s.actual_duration())
            .sum();

        let daily_activity = if is_multi_day {
            self.calculate_daily_activity(&sessions).await?
        } else {
            Vec::new()
        };

        Ok(Stats::new(
            total_sessions,
            total_breaks,
            total_focus_time,
            total_break_time,
            Self::calculate_most_concentrated_period(&sessions),
            Self::calculate_least_concentrated_period(&sessions),
            Self::calculate_concentration_distribution(&sessions),
            self.calculate_category_distribution(&sessions).await?,
            self.calculate_task_distribution(&sessions).await?,
            daily_activity,
        ))
    }

    fn is_multi_day_period(
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> bool {
        match (start_date, end_date) {
            (Some(start), Some(end)) => {
                let duration = end.signed_duration_since(start);
                duration.num_days() > 1
            }
            _ => false,
        }
    }

    fn calculate_most_concentrated_period(sessions: &[FocusSession]) -> ConcentrationPeriod {
        let (morning_total, morning_count, afternoon_total, afternoon_count) =
            sessions.iter().fold((0, 0, 0, 0), |acc, session| {
                if let Some(score) = session.concentration_score() {
                    let hour = session.started_at().hour();
                    if hour < 12 {
                        (acc.0 + score, acc.1 + 1, acc.2, acc.3)
                    } else {
                        (acc.0, acc.1, acc.2 + score, acc.3 + 1)
                    }
                } else {
                    acc
                }
            });

        let morning_avg = if morning_count > 0 {
            morning_total as f64 / morning_count as f64
        } else {
            0.0
        };

        let afternoon_avg = if afternoon_count > 0 {
            afternoon_total as f64 / afternoon_count as f64
        } else {
            0.0
        };

        if morning_avg >= afternoon_avg {
            ConcentrationPeriod::Morning
        } else {
            ConcentrationPeriod::Afternoon
        }
    }

    fn calculate_least_concentrated_period(sessions: &[FocusSession]) -> ConcentrationPeriod {
        let (morning_total, morning_count, afternoon_total, afternoon_count) =
            sessions.iter().fold((0, 0, 0, 0), |acc, session| {
                if let Some(score) = session.concentration_score() {
                    let hour = session.started_at().hour();
                    if hour < 12 {
                        (acc.0 + score, acc.1 + 1, acc.2, acc.3)
                    } else {
                        (acc.0, acc.1, acc.2 + score, acc.3 + 1)
                    }
                } else {
                    acc
                }
            });

        let morning_avg = if morning_count > 0 {
            morning_total as f64 / morning_count as f64
        } else {
            f64::MAX
        };

        let afternoon_avg = if afternoon_count > 0 {
            afternoon_total as f64 / afternoon_count as f64
        } else {
            f64::MAX
        };

        if morning_avg <= afternoon_avg {
            ConcentrationPeriod::Morning
        } else {
            ConcentrationPeriod::Afternoon
        }
    }

    fn calculate_concentration_distribution(sessions: &[FocusSession]) -> [u32; 5] {
        let mut distribution = [0u32; 5];

        for session in sessions {
            if let Some(score) = session.concentration_score() {
                if (1..=5).contains(&score) {
                    distribution[score as usize] += 1;
                }
            }
        }

        distribution
    }

    async fn calculate_category_distribution(
        &self,
        sessions: &[FocusSession],
    ) -> AppResult<Vec<CategoryDistributionItem>> {
        let mut category_times: HashMap<Uuid, i64> = HashMap::new();
        let mut total_time: i64 = 0;

        for session in sessions {
            if let (Some(category_id), Some(duration)) =
                (session.category_id(), session.actual_duration())
            {
                *category_times.entry(category_id).or_insert(0) += duration;
                total_time += duration;
            }
        }

        let category_ids: Vec<Uuid> = category_times.keys().copied().collect();
        let category_futures: Vec<_> = category_ids
            .iter()
            .map(|id| self.category_persistence.find_by_id(*id))
            .collect();

        let categories = join_all(category_futures).await;

        let mut distribution: Vec<CategoryDistributionItem> = category_ids
            .into_iter()
            .zip(categories)
            .filter_map(|(category_id, category_result)| {
                category_result.ok().and_then(|category| {
                    category_times.get(&category_id).map(|&total_focus_time| {
                        let percentage = if total_time > 0 {
                            (total_focus_time as f32 / total_time as f32) * 100.0
                        } else {
                            0.0
                        };

                        CategoryDistributionItem::new(
                            category.name().to_string(),
                            category.id(),
                            total_focus_time,
                            percentage,
                        )
                    })
                })
            })
            .collect();

        distribution.sort_by(|a, b| b.total_focus_time().cmp(&a.total_focus_time()));

        Ok(distribution)
    }

    async fn calculate_task_distribution(
        &self,
        sessions: &[FocusSession],
    ) -> AppResult<Vec<TaskDistributionItem>> {
        let mut task_times: HashMap<Uuid, i64> = HashMap::new();
        let mut total_time: i64 = 0;

        for session in sessions {
            if let (Some(task_id), Some(duration)) = (session.task_id(), session.actual_duration())
            {
                *task_times.entry(task_id).or_insert(0) += duration;
                total_time += duration;
            }
        }

        let task_futures: Vec<_> = task_times
            .keys()
            .map(|task_id| async move {
                let task = self.task_persistence.find_by_id(*task_id).await?;

                let category_name = if let Some(category_id) = task.category_id() {
                    let category = self.category_persistence.find_by_id(category_id).await?;
                    Some(category.name().to_string())
                } else {
                    None
                };

                Ok::<_, crate::application::app_error::AppError>((task, category_name))
            })
            .collect();

        let task_results = join_all(task_futures).await;

        let mut distribution: Vec<TaskDistributionItem> = task_times
            .into_iter()
            .zip(task_results)
            .filter_map(|((_task_id, total_focus_time), result)| {
                result.ok().map(|(task, category_name)| {
                    let percentage = if total_time > 0 {
                        (total_focus_time as f32 / total_time as f32) * 100.0
                    } else {
                        0.0
                    };

                    TaskDistributionItem::new(
                        category_name,
                        task.category_id(),
                        task.name().to_string(),
                        total_focus_time,
                        percentage,
                    )
                })
            })
            .collect();

        distribution.sort_by(|a, b| b.total_focus_time().cmp(&a.total_focus_time()));

        Ok(distribution)
    }

    async fn calculate_daily_activity(
        &self,
        sessions: &[FocusSession],
    ) -> AppResult<Vec<DailyActivityItem>> {
        let mut daily_data: HashMap<NaiveDate, HashMap<Uuid, i64>> = HashMap::new();

        for session in sessions {
            if let (Some(category_id), Some(duration)) =
                (session.category_id(), session.actual_duration())
            {
                let date = session.started_at().date_naive();
                daily_data
                    .entry(date)
                    .or_default()
                    .entry(category_id)
                    .and_modify(|time| *time += duration)
                    .or_insert(duration);
            }
        }

        let unique_category_ids: Vec<Uuid> = daily_data
            .values()
            .flat_map(|categories| categories.keys().copied())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let category_futures: Vec<_> = unique_category_ids
            .iter()
            .map(|id| self.category_persistence.find_by_id(*id))
            .collect();

        let categories = join_all(category_futures).await;

        let category_names: HashMap<Uuid, String> = unique_category_ids
            .into_iter()
            .zip(categories)
            .filter_map(|(id, result)| result.ok().map(|cat| (id, cat.name().to_string())))
            .collect();

        let mut daily_activity: Vec<DailyActivityItem> = daily_data
            .into_iter()
            .map(|(date, categories)| {
                let mut category_distribution: Vec<DailyActivityDistributionItem> = categories
                    .into_iter()
                    .filter_map(|(category_id, total_focus_time)| {
                        category_names.get(&category_id).map(|name| {
                            DailyActivityDistributionItem::new(
                                name.clone(),
                                category_id,
                                total_focus_time,
                            )
                        })
                    })
                    .collect();

                category_distribution
                    .sort_by(|a, b| b.total_focus_time().cmp(&a.total_focus_time()));

                DailyActivityItem::new(date, category_distribution)
            })
            .collect();

        daily_activity.sort_by_key(|a| a.date());

        Ok(daily_activity)
    }
}
