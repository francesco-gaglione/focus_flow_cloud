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
            CategoryDistributionItem, ConcentrationPeriod, DailyActivityDistributionItem, DailyActivityItem, Stats, TaskDistributionItem
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

        let total_sessions = sessions
            .iter()
            .filter(|s| s.session_type == FocusSessionType::Work)
            .count();

        let total_breaks = sessions
            .iter()
            .filter(|s| s.session_type != FocusSessionType::Work)
            .count();

        let total_focus_time: i64 = sessions
            .iter()
            .map(|s| s.actual_duration_minutes.unwrap_or(0) * 60)
            .sum();

        let total_break_time: i64 = sessions
            .iter()
            .map(|s| s.actual_duration_minutes.unwrap_or(0))
            .sum();

        Ok(Stats {
            total_sessions,
            total_breaks,
            total_focus_time,
            total_break_time,
            most_concentrated_period: Self::calculate_most_concentrated_period(&sessions),
            less_concentrated_period: Self::calculate_least_concentrated_period(&sessions),
            concentration_distribution: Self::calculate_concentration_distribution(&sessions)
                .into(),
            category_distribution: self.calculate_category_distribution(&sessions).await?,
            task_distribution: self.calculate_task_distribution(&sessions).await?,
            daily_activity: self.calculate_daily_activity(&sessions).await?,
        })
    }

    fn calculate_most_concentrated_period(sessions: &Vec<FocusSession>) -> ConcentrationPeriod {
        let mut morning_total = 0;
        let mut morning_count = 0;
        let mut afternoon_total = 0;
        let mut afternoon_count = 0;

        for session in sessions {
            if let Some(score) = session.concentration_score {
                let hour = session.started_at.hour();

                if hour < 12 {
                    morning_total += score;
                    morning_count += 1;
                } else {
                    afternoon_total += score;
                    afternoon_count += 1;
                }
            }
        }

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

    fn calculate_least_concentrated_period(sessions: &Vec<FocusSession>) -> ConcentrationPeriod {
        let mut morning_total = 0;
        let mut morning_count = 0;
        let mut afternoon_total = 0;
        let mut afternoon_count = 0;

        for session in sessions {
            if let Some(score) = session.concentration_score {
                let hour = session.started_at.hour();

                if hour < 12 {
                    morning_total += score;
                    morning_count += 1;
                } else {
                    afternoon_total += score;
                    afternoon_count += 1;
                }
            }
        }

        let morning_avg = if morning_count > 0 {
            morning_total as f64 / morning_count as f64
        } else {
            f64::MAX // Se non ci sono dati, considera la media infinita
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

    fn calculate_concentration_distribution(sessions: &Vec<FocusSession>) -> [u32; 11] {
        let mut distribution = [0u32; 11];

        for session in sessions {
            if let Some(score) = session.concentration_score {
                if score >= 0 && score <= 10 {
                    distribution[score as usize] += 1;
                }
            }
        }

        distribution
    }

    async fn calculate_category_distribution(
        &self,
        sessions: &Vec<FocusSession>,
    ) -> AppResult<Vec<CategoryDistributionItem>> {
        let mut category_times: HashMap<Uuid, i64> = HashMap::new();
        let mut total_time: i64 = 0;

        for session in sessions {
            if let (Some(category_id), Some(duration)) =
                (session.category_id, session.actual_duration_minutes)
            {
                *category_times.entry(category_id).or_insert(0) += duration * 60;
                total_time += duration * 60;
            }
        }

        let category_futures: Vec<_> = category_times
            .iter()
            .map(|(category_id, _)| self.category_persistence.find_by_id(*category_id))
            .collect();

        let categories = join_all(category_futures).await;

        let mut distribution: Vec<CategoryDistributionItem> = category_times
            .into_iter()
            .zip(categories)
            .filter_map(|((category_id, total_focus_time), category_result)| {
                category_result.ok().map(|category| {
                    let percentage = if total_time > 0 {
                        (total_focus_time as f32 / total_time as f32) * 100.0
                    } else {
                        0.0
                    };

                    CategoryDistributionItem {
                        category_name: category.name,
                        total_focus_time,
                        percentage,
                    }
                })
            })
            .collect();

        distribution.sort_by(|a, b| b.total_focus_time.cmp(&a.total_focus_time));

        Ok(distribution)
    }

    async fn calculate_task_distribution(
        &self,
        sessions: &Vec<FocusSession>,
    ) -> AppResult<Vec<TaskDistributionItem>> {
        let mut task_times: HashMap<Uuid, i64> = HashMap::new();
        let mut total_time: i64 = 0;

        for session in sessions {
            if let (Some(task_id), Some(duration)) =
                (session.task_id, session.actual_duration_minutes)
            {
                *task_times.entry(task_id).or_insert(0) += duration * 60;
                total_time += duration * 60;
            }
        }

        let mut distribution: Vec<TaskDistributionItem> = Vec::new();

        for (task_id, total_focus_time) in task_times {
            let percentage = if total_time > 0 {
                (total_focus_time as f32 / total_time as f32) * 100.0
            } else {
                0.0
            };

            let task = self.task_persistence.find_by_id(task_id).await?;

            let category = match task.category_id {
                Some(category_id) => {
                    let category = self.category_persistence.find_by_id(category_id).await?;
                    Some(category.name)
                }
                None => None,
            };

            distribution.push(TaskDistributionItem {
                category_name: category,
                task_name: task.name,
                total_focus_time,
                percentage,
            });
        }

        distribution.sort_by(|a, b| b.total_focus_time.cmp(&a.total_focus_time));

        Ok(distribution)
    }

    async fn calculate_daily_activity(
        &self,
        sessions: &Vec<FocusSession>,
    ) -> AppResult<Vec<DailyActivityItem>> {
        let mut daily_data: HashMap<NaiveDate, HashMap<Uuid, i64>> = HashMap::new();

        for session in sessions {
            if let (Some(category_id), Some(duration)) =
                (session.category_id, session.actual_duration_minutes)
            {
                let date = session.started_at.date_naive();

                daily_data
                    .entry(date)
                    .or_insert_with(HashMap::new)
                    .entry(category_id)
                    .and_modify(|time| *time += duration * 60)
                    .or_insert(duration * 60);
            }
        }

        let unique_category_ids: Vec<Uuid> = daily_data
            .values()
            .flat_map(|categories| categories.keys())
            .copied()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let mut category_names: HashMap<Uuid, String> = HashMap::new();
        for category_id in unique_category_ids {
            if let Ok(category) = self.category_persistence.find_by_id(category_id).await {
                category_names.insert(category_id, category.name);
            }
        }

        let mut daily_activity: Vec<DailyActivityItem> = daily_data
            .into_iter()
            .map(|(date, categories)| {
                let mut category_distribution: Vec<DailyActivityDistributionItem> = categories
                    .into_iter()
                    .filter_map(|(category_id, total_focus_time)| {
                        category_names
                            .get(&category_id)
                            .map(|name| DailyActivityDistributionItem {
                                category_name: name.clone(),
                                total_focus_time,
                            })
                    })
                    .collect();

                category_distribution.sort_by(|a, b| b.total_focus_time.cmp(&a.total_focus_time));

                DailyActivityItem {
                    date,
                    category_distribution,
                }
            })
            .collect();

        daily_activity.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(daily_activity)
    }
}
