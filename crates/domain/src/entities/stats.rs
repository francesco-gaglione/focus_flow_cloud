use chrono::NaiveDate;
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
    ) -> Self {
        Self {
            total_sessions,
            total_breaks,
            total_focus_time,
            total_break_time,
            most_concentrated_period,
            less_concentrated_period,
            concentration_distribution,
            category_distribution,
            task_distribution,
            daily_activity,
        }
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
    ) -> Self {
        Self {
            category_name,
            category_id,
            total_focus_time,
            percentage,
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
    ) -> Self {
        Self {
            category_name,
            category_id,
            task_name,
            total_focus_time,
            percentage,
        }
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
        let date = NaiveDate::from_ymd(2023, 1, 1);
        let category_distribution = vec![
            DailyActivityDistributionItem::new("Category 1".to_string(), Uuid::new_v4(), 10),
            DailyActivityDistributionItem::new("Category 2".to_string(), Uuid::new_v4(), 20),
        ];
        let item = DailyActivityItem::new(date, category_distribution);

        assert_eq!(item.date(), date);
        assert_eq!(item.category_distribution().len(), 2);
    }
}
