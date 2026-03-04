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

#[cfg(test)]
mod tests {
    use super::*;

    // ── TaskDistributionItem ──────────────────────────────────────────────────

    #[test]
    fn task_new_valid() {
        let item = TaskDistributionItem::new("My Task".to_string(), 300, 50.0).unwrap();
        assert_eq!(item.task_name(), "My Task");
        assert_eq!(item.total_focus_time(), 300);
        assert_eq!(item.percentage(), 50.0);
    }

    #[test]
    fn task_new_boundary_zero() {
        let item = TaskDistributionItem::new("T".to_string(), 0, 0.0).unwrap();
        assert_eq!(item.percentage(), 0.0);
    }

    #[test]
    fn task_new_boundary_100_001() {
        let item = TaskDistributionItem::new("T".to_string(), 100, 100.001).unwrap();
        assert_eq!(item.percentage(), 100.001);
    }

    #[test]
    fn task_new_invalid_above_boundary() {
        let err = TaskDistributionItem::new("T".to_string(), 100, 100.002).unwrap_err();
        assert!(matches!(err, TaskDistributionError::InvalidStatsParam(_)));
        assert!(err.to_string().contains("100.002"));
    }

    #[test]
    fn task_new_invalid_negative() {
        let err = TaskDistributionItem::new("T".to_string(), 100, -0.1).unwrap_err();
        assert!(matches!(err, TaskDistributionError::InvalidStatsParam(_)));
    }

    #[test]
    fn task_clone() {
        let item = TaskDistributionItem::new("T".to_string(), 100, 50.0).unwrap();
        let cloned = item.clone();
        assert_eq!(cloned.task_name(), item.task_name());
        assert_eq!(cloned.total_focus_time(), item.total_focus_time());
        assert_eq!(cloned.percentage(), item.percentage());
    }

    #[test]
    fn task_debug() {
        let item = TaskDistributionItem::new("T".to_string(), 100, 50.0).unwrap();
        assert!(format!("{:?}", item).contains("50.0"));
    }

    #[test]
    fn task_error_display() {
        let err = TaskDistributionError::InvalidStatsParam("bad value".to_string());
        assert_eq!(err.to_string(), "Invalid stats parameter: bad value");
    }

    #[test]
    fn task_error_debug() {
        let err = TaskDistributionError::InvalidStatsParam("x".to_string());
        assert!(format!("{:?}", err).contains("InvalidStatsParam"));
    }

    // ── CategoryDistributionItem ──────────────────────────────────────────────

    fn make_task() -> TaskDistributionItem {
        TaskDistributionItem::new("Task".to_string(), 100, 100.0).unwrap()
    }

    #[test]
    fn category_new_valid() {
        let id = Uuid::new_v4();
        let item =
            CategoryDistributionItem::new("Cat".to_string(), id, 500, 75.0, vec![make_task()])
                .unwrap();
        assert_eq!(item.category_name(), "Cat");
        assert_eq!(item.category_id(), id);
        assert_eq!(item.total_focus_time(), 500);
        assert_eq!(item.percentage(), 75.0);
        assert_eq!(item.task_distribution().len(), 1);
    }

    #[test]
    fn category_new_boundary_zero() {
        let id = Uuid::new_v4();
        let item = CategoryDistributionItem::new("Cat".to_string(), id, 0, 0.0, vec![]).unwrap();
        assert_eq!(item.percentage(), 0.0);
        assert!(item.task_distribution().is_empty());
    }

    #[test]
    fn category_new_boundary_100_001() {
        let id = Uuid::new_v4();
        let item =
            CategoryDistributionItem::new("Cat".to_string(), id, 100, 100.001, vec![]).unwrap();
        assert_eq!(item.percentage(), 100.001);
    }

    #[test]
    fn category_new_invalid_above_boundary() {
        let id = Uuid::new_v4();
        let err =
            CategoryDistributionItem::new("Cat".to_string(), id, 100, 100.002, vec![]).unwrap_err();
        assert!(matches!(err, CategoryDistributionError::InvalidStatsParam(_)));
        assert!(err.to_string().contains("100.002"));
    }

    #[test]
    fn category_new_invalid_negative() {
        let id = Uuid::new_v4();
        let err =
            CategoryDistributionItem::new("Cat".to_string(), id, 100, -1.0, vec![]).unwrap_err();
        assert!(matches!(err, CategoryDistributionError::InvalidStatsParam(_)));
    }

    #[test]
    fn category_clone() {
        let id = Uuid::new_v4();
        let item =
            CategoryDistributionItem::new("Cat".to_string(), id, 100, 50.0, vec![]).unwrap();
        let cloned = item.clone();
        assert_eq!(cloned.category_name(), item.category_name());
        assert_eq!(cloned.category_id(), item.category_id());
        assert_eq!(cloned.total_focus_time(), item.total_focus_time());
        assert_eq!(cloned.percentage(), item.percentage());
    }

    #[test]
    fn category_debug() {
        let id = Uuid::new_v4();
        let item =
            CategoryDistributionItem::new("Cat".to_string(), id, 100, 50.0, vec![]).unwrap();
        assert!(format!("{:?}", item).contains("Cat"));
    }

    #[test]
    fn category_error_display() {
        let err = CategoryDistributionError::InvalidStatsParam("bad value".to_string());
        assert_eq!(err.to_string(), "Invalid stats parameter: bad value");
    }

    #[test]
    fn category_error_debug() {
        let err = CategoryDistributionError::InvalidStatsParam("x".to_string());
        assert!(format!("{:?}", err).contains("InvalidStatsParam"));
    }
}
