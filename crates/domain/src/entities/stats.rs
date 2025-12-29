use super::focus_session::FocusSession;
use super::focus_session_type::FocusSessionType;
use crate::entities::category::Category;
use crate::entities::task::Task;
use crate::error::domain_error::{DomainError, DomainResult};
use chrono::{NaiveDate, Timelike};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Stats {
    total_sessions: usize,
    total_breaks: usize,
    total_focus_time: i64,
    total_break_time: i64,
    most_concentrated_period: ConcentrationPeriod,
    less_concentrated_period: ConcentrationPeriod,
    concentration_distribution: [u32; 5],
    category_distribution: Vec<CategoryDistributionItem>,
    task_distribution: Vec<TaskDistributionItem>,
    daily_activity: Vec<DailyActivityItem>,
}

impl Stats {
    pub fn calculate(
        sessions: &[FocusSession],
        categories: &[Category],
        tasks: &[Task],
        is_multi_day: bool,
    ) -> DomainResult<Self> {
        // Build lookup maps internally
        let category_names: HashMap<Uuid, String> = categories
            .iter()
            .map(|c| (c.id(), c.name().to_string()))
            .collect();

        let task_details: HashMap<Uuid, (String, Option<String>)> = tasks
            .iter()
            .map(|t| {
                let cat_name = t
                    .category_id()
                    .and_then(|id| category_names.get(&id))
                    .cloned();
                (t.id(), (t.name().to_string(), cat_name))
            })
            .collect();

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
            Self::calculate_daily_activity(sessions, &category_names)?
        } else {
            Vec::new()
        };

        Ok(Self {
            total_sessions,
            total_breaks,
            total_focus_time,
            total_break_time,
            most_concentrated_period: Self::calculate_most_concentrated_period(sessions),
            less_concentrated_period: Self::calculate_least_concentrated_period(sessions),
            concentration_distribution: Self::calculate_concentration_distribution(sessions),
            category_distribution: Self::calculate_category_distribution(
                sessions,
                &category_names,
            )?,
            task_distribution: Self::calculate_task_distribution(sessions, &task_details)?,
            daily_activity,
        })
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
                if score > 0 {
                    let index = (score - 1) as usize;
                    if let Some(count) = distribution.get_mut(index) {
                        *count += 1;
                    }
                }
            }
        }

        distribution
    }

    fn calculate_category_distribution(
        sessions: &[FocusSession],
        category_names: &HashMap<Uuid, String>,
    ) -> DomainResult<Vec<CategoryDistributionItem>> {
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

        let mut distribution: Vec<CategoryDistributionItem> = category_times
            .into_iter()
            .filter_map(|(category_id, total_focus_time)| {
                category_names.get(&category_id).map(|name| {
                    let percentage = if total_time > 0 {
                        (total_focus_time as f32 / total_time as f32) * 100.0
                    } else {
                        0.0
                    };

                    CategoryDistributionItem::new(
                        name.to_string(),
                        category_id,
                        total_focus_time,
                        percentage,
                    )
                })
            })
            .collect::<DomainResult<Vec<CategoryDistributionItem>>>()?;

        distribution.sort_by_key(|b| std::cmp::Reverse(b.total_focus_time()));

        Ok(distribution)
    }

    fn calculate_task_distribution(
        sessions: &[FocusSession],
        task_details: &HashMap<Uuid, (String, Option<String>)>,
    ) -> DomainResult<Vec<TaskDistributionItem>> {
        let mut task_times: HashMap<Uuid, i64> = HashMap::new();
        let mut total_time: i64 = 0;

        for session in sessions {
            if let (Some(task_id), Some(duration)) = (session.task_id(), session.actual_duration())
            {
                *task_times.entry(task_id).or_insert(0) += duration;
                total_time += duration;
            }
        }

        let mut distribution: Vec<TaskDistributionItem> = task_times
            .into_iter()
            .filter_map(|(task_id, total_focus_time)| {
                task_details
                    .get(&task_id)
                    .map(|(task_name, category_name)| {
                        let percentage = if total_time > 0 {
                            (total_focus_time as f32 / total_time as f32) * 100.0
                        } else {
                            0.0
                        };

                        TaskDistributionItem::new(
                            category_name.clone(),
                            sessions
                                .iter()
                                .find(|s| s.task_id() == Some(task_id))
                                .and_then(|s| s.category_id()),
                            task_name.clone(),
                            total_focus_time,
                            percentage,
                        )
                    })
            })
            .collect::<DomainResult<Vec<TaskDistributionItem>>>()?;

        distribution.sort_by_key(|b| std::cmp::Reverse(b.total_focus_time()));

        Ok(distribution)
    }

    fn calculate_daily_activity(
        sessions: &[FocusSession],
        category_names: &HashMap<Uuid, String>,
    ) -> DomainResult<Vec<DailyActivityItem>> {
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

                category_distribution.sort_by_key(|b| std::cmp::Reverse(b.total_focus_time()));

                Ok(DailyActivityItem::new(date, category_distribution))
            })
            .collect::<DomainResult<Vec<DailyActivityItem>>>()?;

        daily_activity.sort_by_key(|a| a.date());

        Ok(daily_activity)
    }

    pub fn total_sessions(&self) -> usize {
        self.total_sessions
    }

    pub fn total_breaks(&self) -> usize {
        self.total_breaks
    }

    pub fn total_focus_time(&self) -> i64 {
        self.total_focus_time
    }

    pub fn total_break_time(&self) -> i64 {
        self.total_break_time
    }

    pub fn most_concentrated_period(&self) -> &ConcentrationPeriod {
        &self.most_concentrated_period
    }

    pub fn less_concentrated_period(&self) -> &ConcentrationPeriod {
        &self.less_concentrated_period
    }

    pub fn concentration_distribution(&self) -> &[u32; 5] {
        &self.concentration_distribution
    }

    pub fn category_distribution(&self) -> &[CategoryDistributionItem] {
        &self.category_distribution
    }

    pub fn task_distribution(&self) -> &[TaskDistributionItem] {
        &self.task_distribution
    }

    pub fn daily_activity(&self) -> &[DailyActivityItem] {
        &self.daily_activity
    }
}

#[derive(Debug, Clone)]
pub enum ConcentrationPeriod {
    Morning,
    Afternoon,
}

#[derive(Debug, Clone)]
pub struct CategoryDistributionItem {
    category_name: String,
    category_id: Uuid,
    total_focus_time: i64,
    percentage: f32,
}

impl CategoryDistributionItem {
    pub fn new(
        category_name: String,
        category_id: Uuid,
        total_focus_time: i64,
        percentage: f32,
    ) -> DomainResult<Self> {
        if !(0.0..=100.001).contains(&percentage) {
            return Err(DomainError::InvalidStatsParam(format!(
                "Percentage must be between 0 and 100, got {}",
                percentage
            )));
        }
        Ok(Self {
            category_name,
            category_id,
            total_focus_time,
            percentage,
        })
    }

    pub fn category_name(&self) -> &str {
        &self.category_name
    }

    pub fn category_id(&self) -> Uuid {
        self.category_id
    }

    pub fn total_focus_time(&self) -> i64 {
        self.total_focus_time
    }

    pub fn percentage(&self) -> f32 {
        self.percentage
    }
}

#[derive(Debug, Clone)]
pub struct TaskDistributionItem {
    category_name: Option<String>,
    category_id: Option<Uuid>,
    task_name: String,
    total_focus_time: i64,
    percentage: f32,
}

impl TaskDistributionItem {
    pub fn new(
        category_name: Option<String>,
        category_id: Option<Uuid>,
        task_name: String,
        total_focus_time: i64,
        percentage: f32,
    ) -> DomainResult<Self> {
        if !(0.0..=100.001).contains(&percentage) {
            return Err(DomainError::InvalidStatsParam(format!(
                "Percentage must be between 0 and 100, got {}",
                percentage
            )));
        }
        Ok(Self {
            category_name,
            category_id,
            task_name,
            total_focus_time,
            percentage,
        })
    }

    pub fn category_name(&self) -> Option<&str> {
        self.category_name.as_deref()
    }

    pub fn category_id(&self) -> Option<Uuid> {
        self.category_id
    }

    pub fn task_name(&self) -> &str {
        &self.task_name
    }

    pub fn total_focus_time(&self) -> i64 {
        self.total_focus_time
    }

    pub fn percentage(&self) -> f32 {
        self.percentage
    }
}

#[derive(Debug, Clone)]
pub struct DailyActivityItem {
    date: NaiveDate,
    category_distribution: Vec<DailyActivityDistributionItem>,
}

impl DailyActivityItem {
    pub fn new(date: NaiveDate, category_distribution: Vec<DailyActivityDistributionItem>) -> Self {
        Self {
            date,
            category_distribution,
        }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn category_distribution(&self) -> &[DailyActivityDistributionItem] {
        &self.category_distribution
    }
}

#[derive(Debug, Clone)]
pub struct DailyActivityDistributionItem {
    category_name: String,
    category_id: Uuid,
    total_focus_time: i64,
}

impl DailyActivityDistributionItem {
    pub fn new(category_name: String, category_id: Uuid, total_focus_time: i64) -> Self {
        Self {
            category_name,
            category_id,
            total_focus_time,
        }
    }

    pub fn category_name(&self) -> &str {
        &self.category_name
    }

    pub fn category_id(&self) -> Uuid {
        self.category_id
    }

    pub fn total_focus_time(&self) -> i64 {
        self.total_focus_time
    }
}

#[cfg(test)]
mod stats_tests {
    use super::*;

    #[test]
    fn test_daily_activity_distribution_item_new() {
        let category_name = "Category 1".to_string();
        let category_id = Uuid::new_v4();
        let total_focus_time = 10;
        let item = DailyActivityDistributionItem::new(
            category_name.clone(),
            category_id,
            total_focus_time,
        );

        assert_eq!(item.category_name(), category_name);
        assert_eq!(item.category_id(), category_id);
        assert_eq!(item.total_focus_time(), total_focus_time);
    }

    #[test]
    fn test_daily_activity_item_new() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let category_distribution = vec![
            DailyActivityDistributionItem::new("Category 1".to_string(), Uuid::new_v4(), 10),
            DailyActivityDistributionItem::new("Category 2".to_string(), Uuid::new_v4(), 20),
        ];
        let item = DailyActivityItem::new(date, category_distribution);

        assert_eq!(item.date(), date);
        assert_eq!(item.category_distribution().len(), 2);
    }
}
