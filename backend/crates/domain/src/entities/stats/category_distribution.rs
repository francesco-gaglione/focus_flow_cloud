use uuid::Uuid;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CategoryDistributionError {
    #[error("Invalid stats parameter: {0}")]
    InvalidStatsParam(String),
}

pub type CategoryDistributionResult<T> = Result<T, CategoryDistributionError>;

#[derive(Debug, Error)]
pub enum TaskDistributionError {
    #[error("Invalid stats parameter: {0}")]
    InvalidStatsParam(String),
}

pub type TaskDistributionResult<T> = Result<T, TaskDistributionError>;

// Represents the total time spent on each category and the percentage of total time spent on each category, with tasks details.
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
    ) -> CategoryDistributionResult<Self> {
        if !(0.0..=100.001).contains(&percentage) {
            return Err(CategoryDistributionError::InvalidStatsParam(format!(
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
    pub fn new(
        task_name: String,
        total_focus_time: i64,
        percentage: f32,
    ) -> TaskDistributionResult<Self> {
        if !(0.0..=100.001).contains(&percentage) {
            return Err(TaskDistributionError::InvalidStatsParam(format!(
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
