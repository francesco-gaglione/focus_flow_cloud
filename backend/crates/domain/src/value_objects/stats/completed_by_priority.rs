use std::collections::HashMap;

use crate::entities::tasks::task_priority::TaskPriority;

pub struct CompletedByPriority {
    counts: HashMap<TaskPriority, usize>,
}

impl CompletedByPriority {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn update_count(&mut self, priority: TaskPriority, count: usize) {
        *self.counts.entry(priority).or_default() += count;
    }

    pub fn get_count(&self, priority: TaskPriority) -> usize {
        *self.counts.get(&priority).unwrap_or(&0)
    }

    pub fn increment_count(&mut self, priority: TaskPriority, count: usize) {
        *self.counts.entry(priority).or_default() += count;
    }

    pub fn decrement_count(&mut self, priority: TaskPriority, count: usize) {
        if let Some(c) = self.counts.get_mut(&priority) {
            *c -= count;
        }
    }
}

#[cfg(test)]
pub mod completed_by_priority_tests {
    use crate::entities::tasks::task_priority::TaskPriority;

    use super::*;

    #[test]
    fn test_creation() {
        let mut completed_by_priority = CompletedByPriority::new();
        completed_by_priority.update_count(TaskPriority::Low, 1);
        completed_by_priority.update_count(TaskPriority::Medium, 2);
        completed_by_priority.update_count(TaskPriority::High, 3);
        completed_by_priority.update_count(TaskPriority::Urgent, 4);
        assert_eq!(completed_by_priority.get_count(TaskPriority::Low), 1);
        assert_eq!(completed_by_priority.get_count(TaskPriority::Medium), 2);
        assert_eq!(completed_by_priority.get_count(TaskPriority::High), 3);
        assert_eq!(completed_by_priority.get_count(TaskPriority::Urgent), 4);
    }

    #[test]
    fn test_increment() {
        let mut completed_by_priority = CompletedByPriority::new();
        completed_by_priority.update_count(TaskPriority::Low, 1);
        completed_by_priority.update_count(TaskPriority::Medium, 2);
        completed_by_priority.update_count(TaskPriority::High, 3);
        completed_by_priority.update_count(TaskPriority::Urgent, 4);
        assert_eq!(completed_by_priority.get_count(TaskPriority::Low), 1);
        assert_eq!(completed_by_priority.get_count(TaskPriority::Medium), 2);
        assert_eq!(completed_by_priority.get_count(TaskPriority::High), 3);
        assert_eq!(completed_by_priority.get_count(TaskPriority::Urgent), 4);
        completed_by_priority.increment_count(TaskPriority::Low, 2);
        assert_eq!(completed_by_priority.get_count(TaskPriority::Low), 3);
    }

    #[test]
    fn test_decrement() {
        let mut completed_by_priority = CompletedByPriority::new();
        completed_by_priority.update_count(TaskPriority::Low, 3);
        completed_by_priority.decrement_count(TaskPriority::Low, 2);
        assert_eq!(completed_by_priority.get_count(TaskPriority::Low), 1);
    }
}
