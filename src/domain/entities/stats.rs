use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Stats {
    pub total_sessions: usize,
    pub total_breaks: usize,
    pub total_focus_time: i64,
    pub total_break_time: i64,
    pub most_concentrated_period: ConcentrationPeriod,
    pub less_concentrated_period: ConcentrationPeriod,
    pub concentration_distribution: [u32; 5], // vec of 5 elements rapresenting the number of session of each 5 rating
    pub category_distribution: Vec<CategoryDistributionItem>,
    pub task_distribution: Vec<TaskDistributionItem>,
    pub daily_activity: Vec<DailyActivityItem>,
}

#[derive(Debug, Clone)]
pub enum ConcentrationPeriod {
    Morning,
    Afternoon,
}

#[derive(Debug, Clone)]
pub struct CategoryDistributionItem {
    pub category_name: String,
    pub category_id: Uuid,
    pub total_focus_time: i64,
    pub percentage: f32,
}

#[derive(Debug, Clone)]
pub struct TaskDistributionItem {
    pub category_name: Option<String>,
    pub category_id: Option<Uuid>,
    pub task_name: String,
    pub total_focus_time: i64,
    pub percentage: f32,
}

#[derive(Debug, Clone)]
pub struct DailyActivityItem {
    pub date: NaiveDate,
    pub category_distribution: Vec<DailyActivityDistributionItem>,
}

#[derive(Debug, Clone)]
pub struct DailyActivityDistributionItem {
    pub category_name: String,
    pub category_id: Uuid,
    pub total_focus_time: i64,
}
