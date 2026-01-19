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

        // Task id - Task name
        let task_details: HashMap<Uuid, String> = tasks
            .iter()
            .map(|t| (t.id(), t.name().to_string()))
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
                &task_details,
            )?,
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
        task_details: &HashMap<Uuid, String>,
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

                    // Filter sessions for this category to ensure we only list relevant tasks
                    let category_sessions: Vec<FocusSession> = sessions
                        .iter()
                        .filter(|s| s.category_id() == Some(category_id))
                        .cloned()
                        .collect();

                    let task_distribution = Self::calculate_task_distribution(
                        &category_sessions,
                        task_details,
                        total_time,
                    )?;

                    CategoryDistributionItem::new(
                        name.to_string(),
                        category_id,
                        total_focus_time,
                        percentage,
                        task_distribution,
                    )
                })
            })
            .collect::<DomainResult<Vec<CategoryDistributionItem>>>()?;

        distribution.sort_by_key(|b| std::cmp::Reverse(b.total_focus_time()));

        Ok(distribution)
    }

    fn calculate_task_distribution(
        sessions: &[FocusSession],
        task_details: &HashMap<Uuid, String>, // taskId - task_name
        total_time: i64,                      // filtered period total time
    ) -> DomainResult<Vec<TaskDistributionItem>> {
        let mut task_times: HashMap<Uuid, i64> = HashMap::new();

        for session in sessions {
            if let (Some(task_id), Some(duration)) = (session.task_id(), session.actual_duration())
            {
                *task_times.entry(task_id).or_insert(0) += duration;
            }
        }

        let mut distribution: Vec<TaskDistributionItem> = task_times
            .into_iter()
            .filter_map(|(task_id, total_focus_time)| {
                task_details.get(&task_id).map(|task_name| {
                    let percentage = if total_time > 0 {
                        (total_focus_time as f32 / total_time as f32) * 100.0
                    } else {
                        0.0
                    };

                    TaskDistributionItem::new(task_name.clone(), total_focus_time, percentage)
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
    task_distribution: Vec<TaskDistributionItem>,
}

impl CategoryDistributionItem {
    pub fn new(
        category_name: String,
        category_id: Uuid,
        total_focus_time: i64,
        percentage: f32,
        task_distribution: Vec<TaskDistributionItem>,
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
            task_distribution,
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

    pub fn task_distribution(&self) -> &Vec<TaskDistributionItem> {
        &self.task_distribution
    }
}

#[derive(Debug, Clone)]
pub struct TaskDistributionItem {
    task_name: String,
    total_focus_time: i64,
    percentage: f32,
}

impl TaskDistributionItem {
    pub fn new(task_name: String, total_focus_time: i64, percentage: f32) -> DomainResult<Self> {
        if !(0.0..=100.001).contains(&percentage) {
            return Err(DomainError::InvalidStatsParam(format!(
                "Percentage must be between 0 and 100, got {}",
                percentage
            )));
        }

        Ok(Self {
            task_name,
            total_focus_time,
            percentage,
        })
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
    use chrono::{DateTime, Utc};

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

    #[test]
    fn test_stats_calculation_correctness() {
        // Fixed date: 2023-01-01 10:00:00 UTC (Morning)
        let date = DateTime::parse_from_rfc3339("2023-01-01T10:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        // Session 1 duration: 1 hour (3600 seconds)
        let end_date_1 = date + chrono::Duration::seconds(3600);

        // Session 2 duration: 2 hours (7200 seconds)
        let end_date_2 = date + chrono::Duration::seconds(7200);

        let categories = vec![
            Category::create(
                Uuid::new_v4(),
                "Category 1".to_string(),
                Some("des".to_string()),
                "#000000".to_string(),
            )
            .unwrap(),
            Category::create(
                Uuid::new_v4(),
                "Category 2".to_string(),
                Some("des".to_string()),
                "#000000".to_string(),
            )
            .unwrap(),
        ];

        let tasks = vec![
            Task::create(
                Uuid::new_v4(),
                Some(categories.get(0).unwrap().id()),
                "Task 1".to_string(),
                Some("des".to_string()),
                None,
            ),
            Task::create(
                Uuid::new_v4(),
                Some(categories.get(1).unwrap().id()),
                "Task 2".to_string(),
                Some("des".to_string()),
                None,
            ),
        ];

        let focus_sessions = vec![
            FocusSession::new(
                categories.get(0).unwrap().user_id(),
                Some(categories.get(0).unwrap().id()),
                Some(tasks.get(0).unwrap().id()),
                FocusSessionType::Work,
                Some(3),
                Some("notes cat 1, task 1".to_string()),
                date,
                Some(end_date_1),
            )
            .unwrap(),
            FocusSession::new(
                categories.get(1).unwrap().user_id(),
                Some(categories.get(1).unwrap().id()),
                Some(tasks.get(1).unwrap().id()),
                FocusSessionType::Work,
                Some(3),
                Some("notes cat 2, task 2".to_string()),
                date,
                Some(end_date_2),
            )
            .unwrap(),
        ];

        let stats = Stats::calculate(&focus_sessions, &categories, &tasks, false);
        assert!(stats.is_ok());

        let stats = stats.unwrap();
        assert_eq!(stats.total_sessions(), 2);
        assert_eq!(stats.total_breaks(), 0);
        assert_eq!(stats.total_focus_time(), 3600 + 7200);
        assert_eq!(stats.total_break_time(), 0);

        // Both sessions are in the morning (10 AM), so Morning is most concentrated
        assert!(matches!(
            stats.most_concentrated_period(),
            ConcentrationPeriod::Morning
        ));

        // Since there are no afternoon sessions, afternoon average is MAX, so Morning (avg 3) < MAX.
        // Thus Morning is also the "least" concentrated when compared to "no data" treated as MAX.
        assert!(matches!(
            stats.less_concentrated_period(),
            ConcentrationPeriod::Morning
        ));

        // Concentration distribution: both have score 3 (index 2)
        assert_eq!(stats.concentration_distribution(), &[0, 0, 2, 0, 0]);

        // Category distribution
        let cat_dist = stats.category_distribution();
        assert_eq!(cat_dist.len(), 2);

        // Sorted by focus time desc: Category 2 (7200) should be first
        assert_eq!(cat_dist[0].category_name(), "Category 2");
        assert_eq!(cat_dist[0].total_focus_time(), 7200);

        // 7200 / 10800 = 0.666...
        assert!((cat_dist[0].percentage() - 66.66).abs() < 0.01);
        assert_eq!(cat_dist[1].category_name(), "Category 1");
        assert_eq!(cat_dist[1].total_focus_time(), 3600);

        // 3600 / 10800 = 0.333...
        assert!((cat_dist[1].percentage() - 33.33).abs() < 0.01);

        // Daily activity is empty because is_multi_day is false
        assert!(stats.daily_activity().is_empty());
    }

    #[test]
    fn test_stats_multi_day_and_distributions() {
        // Day 1: 2023-01-01
        let day1_start = DateTime::parse_from_rfc3339("2023-01-01T10:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        let day1_end = day1_start + chrono::Duration::seconds(3600); // 1 hour

        // Day 2: 2023-01-02
        let day2_start = DateTime::parse_from_rfc3339("2023-01-02T15:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let day2_end = day2_start + chrono::Duration::seconds(7200); // 2 hours
        let categories = vec![
            Category::create(
                Uuid::new_v4(),
                "Cat A".to_string(),
                None,
                "#ffffff".to_string(),
            )
            .unwrap(),
            Category::create(
                Uuid::new_v4(),
                "Cat B".to_string(),
                None,
                "#000000".to_string(),
            )
            .unwrap(),
        ];

        let tasks = vec![
            Task::create(
                Uuid::new_v4(),
                Some(categories[0].id()),
                "Task A1".to_string(),
                None,
                None,
            ),
            Task::create(
                Uuid::new_v4(),
                Some(categories[1].id()),
                "Task B1".to_string(),
                None,
                None,
            ),
        ];

        let sessions = vec![
            // Session 1: Cat A, Task A1, 1 hour, Day 1
            FocusSession::new(
                Uuid::new_v4(),
                Some(categories[0].id()),
                Some(tasks[0].id()),
                FocusSessionType::Work,
                Some(5),
                None,
                day1_start,
                Some(day1_end),
            )
            .unwrap(),
            // Session 2: Cat B, Task B1, 2 hours, Day 2
            FocusSession::new(
                Uuid::new_v4(),
                Some(categories[1].id()),
                Some(tasks[1].id()),
                FocusSessionType::Work,
                Some(4),
                None,
                day2_start,
                Some(day2_end),
            )
            .unwrap(),
        ];

        let stats = Stats::calculate(&sessions, &categories, &tasks, true).unwrap();

        // Verify Daily Activity
        let daily = stats.daily_activity();
        assert_eq!(daily.len(), 2, "Should have 2 days of activity");

        // Day 1 check
        assert_eq!(daily[0].date(), day1_start.date_naive());
        assert_eq!(daily[0].category_distribution().len(), 1);
        assert_eq!(daily[0].category_distribution()[0].category_name(), "Cat A");
        assert_eq!(daily[0].category_distribution()[0].total_focus_time(), 3600);

        // Day 2 check
        assert_eq!(daily[1].date(), day2_start.date_naive());
        assert_eq!(daily[1].category_distribution().len(), 1);
        assert_eq!(daily[1].category_distribution()[0].category_name(), "Cat B");
        assert_eq!(daily[1].category_distribution()[0].total_focus_time(), 7200);

        // Verify Global Task Distribution Percentages
        // Total Time = 3600 + 7200 = 10800
        // Task A1: 3600 -> 33.33%
        // Task B1: 7200 -> 66.66%
        let cat_dist = stats.category_distribution();

        // Find Cat A
        let cat_a_dist = cat_dist
            .iter()
            .find(|c| c.category_name() == "Cat A")
            .expect("Cat A not found");
        let task_a1_dist = &cat_a_dist.task_distribution();

        // Cat A should ONLY contain Task A1
        assert_eq!(task_a1_dist.len(), 1, "Category A should only list Task A1");
        assert_eq!(task_a1_dist[0].task_name(), "Task A1");

        // CRITICAL: The percentage should be relative to GLOBAL total (10800), NOT Category total (3600)
        // 3600 / 10800 = 33.33%
        assert!(
            (task_a1_dist[0].percentage() - 33.33).abs() < 0.01,
            "Task A1 % should be ~33.33%, got {}",
            task_a1_dist[0].percentage()
        );

        // Find Cat B
        let cat_b_dist = cat_dist
            .iter()
            .find(|c| c.category_name() == "Cat B")
            .expect("Cat B not found");

        let task_b1_dist = &cat_b_dist.task_distribution();

        // Cat B should ONLY contain Task B1
        assert_eq!(task_b1_dist.len(), 1, "Category B should only list Task B1");
        assert_eq!(task_b1_dist[0].task_name(), "Task B1");

        // 7200 / 10800 = 66.66%
        assert!(
            (task_b1_dist[0].percentage() - 66.66).abs() < 0.01,
            "Task B1 % should be ~66.66%, got {}",
            task_b1_dist[0].percentage()
        );
    }
}
