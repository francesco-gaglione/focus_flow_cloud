use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::entities::{
    reminder::Reminder,
    tasks::{subtask::Subtask, task_priority::TaskPriority},
};

#[derive(Debug, Clone, Error, PartialEq)]
pub enum TaskError {
    #[error("sub-tasks must be completed before marking task as completed")]
    UncompletedSubTasks,
}

pub type TaskResult<T> = Result<T, TaskError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: Option<String>,
    priority: Option<TaskPriority>,
    sub_tasks: Vec<Subtask>,
    due_date: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    reminders: Vec<Reminder>,
}

impl Task {
    // --- constructors ---

    pub fn new(
        user_id: Uuid,
        title: String,
        due_date: Option<DateTime<Utc>>,
        description: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            description,
            priority: None,
            sub_tasks: Vec::new(),
            due_date,
            completed_at: None,
            reminders: Vec::new(),
        }
    }

    pub fn reconstitute(
        id: Uuid,
        user_id: Uuid,
        title: String,
        description: Option<String>,
        priority: Option<TaskPriority>,
        sub_tasks: Vec<Subtask>,
        due_date: Option<DateTime<Utc>>,
        completed_at: Option<DateTime<Utc>>,
        reminders: Vec<Reminder>,
    ) -> Self {
        Self {
            id,
            user_id,
            title,
            description,
            priority,
            sub_tasks,
            due_date,
            completed_at,
            reminders,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn is_overdue(&self) -> bool {
        self.due_date
            .map(|d| d < Utc::now() && !self.is_completed())
            .unwrap_or(false)
    }

    pub fn overdue_reminders(&self) -> Vec<&Reminder> {
        let now = Utc::now();
        self.reminders
            .iter()
            .filter(|r| !r.is_sent() && r.date() <= now)
            .collect()
    }

    pub fn overdue_reminders_mut(&mut self) -> Vec<&mut Reminder> {
        let now = Utc::now();
        self.reminders
            .iter_mut()
            .filter(|r| !r.is_sent() && r.date() <= now)
            .collect()
    }

    pub fn complete(&mut self) -> TaskResult<()> {
        if self.sub_tasks.iter().any(|s| !s.is_completed()) {
            return Err(TaskError::UncompletedSubTasks);
        }
        self.completed_at = Some(Utc::now());
        Ok(())
    }

    pub fn mark_completed(&mut self) -> TaskResult<()> {
        self.complete()
    }

    pub fn update_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn update_due_date(&mut self, due_date: Option<DateTime<Utc>>) {
        self.due_date = due_date;
    }

    pub fn update_priority(&mut self, priority: Option<TaskPriority>) {
        self.priority = priority;
    }

    pub fn update_user_id(&mut self, user_id: Uuid) {
        self.user_id = user_id;
    }

    pub fn set_priority(&mut self, priority: TaskPriority) {
        self.priority = Some(priority);
    }

    pub fn set_due_date(&mut self, due_date: Option<DateTime<Utc>>) {
        self.due_date = due_date;
    }

    pub fn add_subtask(&mut self, subtask: Subtask) {
        self.sub_tasks.push(subtask);
    }

    pub fn add_reminder(&mut self, reminder: Reminder) {
        self.reminders.push(reminder);
    }

    // Getters

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn priority(&self) -> Option<TaskPriority> {
        self.priority
    }

    pub fn due_date(&self) -> Option<DateTime<Utc>> {
        self.due_date
    }

    pub fn completed_at(&self) -> Option<DateTime<Utc>> {
        self.completed_at
    }

    pub fn completed(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn reminders(&self) -> &[Reminder] {
        &self.reminders
    }

    pub fn sub_tasks(&self) -> &[Subtask] {
        &self.sub_tasks
    }

    pub fn sub_tasks_mut(&mut self) -> &mut Vec<Subtask> {
        &mut self.sub_tasks
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::Utc;
    use uuid::Uuid;

    use crate::entities::{
        reminder::Reminder,
        tasks::{subtask::Subtask, task::Task, task_priority::TaskPriority},
    };

    #[test]
    fn test_task_creation() {
        let user_id = Uuid::new_v4();
        let title = "Test Task";
        let date = Utc::now() + Duration::from_mins(10);
        let description = "Test description";
        let task = Task::new(
            user_id,
            title.to_string(),
            Some(date),
            Some(description.to_string()),
        );
        assert_eq!(task.title(), title);
        assert_eq!(task.due_date(), Some(date));
        assert_eq!(task.description(), Some(description));
    }

    #[test]
    fn test_task_getters() {
        let user_id = Uuid::new_v4();
        let title = "Test Task";
        let date = Utc::now() + Duration::from_mins(10);
        let description = "Test description";
        let task = Task::new(
            user_id,
            title.to_string(),
            Some(date),
            Some(description.to_string()),
        );
        assert_eq!(task.title(), title);
        assert_eq!(task.due_date(), Some(date));
        assert_eq!(task.description(), Some(description));
        assert_eq!(task.priority(), None);
        assert_eq!(task.completed(), false);
        assert_eq!(task.reminders(), &[]);
    }

    #[test]
    fn test_task_updates() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        task.update_title("Updated Task".to_string());
        task.update_description(Some("Updated description".to_string()));
        assert_eq!(task.title(), "Updated Task");
        assert_eq!(task.description(), Some("Updated description"));
        task.update_priority(Some(TaskPriority::High));
        assert_eq!(task.priority(), Some(TaskPriority::High));
        let new_due = Utc::now() + Duration::from_mins(10);
        task.update_due_date(Some(new_due));
        assert_eq!(task.due_date(), Some(new_due));
        task.mark_completed().unwrap();
        assert_eq!(task.completed(), true);
        let new_user_id = Uuid::new_v4();
        task.update_user_id(new_user_id);
        assert_eq!(task.user_id(), new_user_id);
    }

    #[test]
    fn test_task_subtasks() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        let subtask = Subtask::new("Subtask 1".to_string(), None);
        task.add_subtask(subtask);
        assert_eq!(task.sub_tasks().len(), 1);
        let sub_tasks = task.sub_tasks();
        assert_eq!(sub_tasks.len(), 1);
        assert_eq!(sub_tasks[0].title(), "Subtask 1");
    }

    #[test]
    fn test_task_subtask_deletion() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        let subtask = Subtask::new("Subtask 1".to_string(), None);
        task.add_subtask(subtask);
        assert_eq!(task.sub_tasks().len(), 1);
        task.sub_tasks_mut().retain(|s| s.title != "Subtask 1");
        assert_eq!(task.sub_tasks().len(), 0);
    }

    #[test]
    fn test_task_subtask_update() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        let subtask = Subtask::new("Subtask 1".to_string(), None);
        task.add_subtask(subtask);
        assert_eq!(task.sub_tasks().len(), 1);
        task.sub_tasks_mut()[0].update_title("Edited Subtask 1".to_string());
        assert_eq!(task.sub_tasks()[0].title(), "Edited Subtask 1");
    }

    #[test]
    fn test_task_subtask_completion() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        let subtask = Subtask::new("Subtask 1".to_string(), None);
        task.add_subtask(subtask);
        task.sub_tasks_mut()[0].mark_completed();
        assert!(task.sub_tasks()[0].is_completed());
        assert!(!task.is_completed());
    }

    #[test]
    fn test_task_completion() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        let subtask = Subtask::new("Subtask 1".to_string(), None);
        task.add_subtask(subtask);
        let res = task.complete();
        assert!(res.is_err());
        assert!(!task.is_completed());
        task.sub_tasks_mut()[0].mark_completed();
        let res = task.complete();
        assert!(res.is_ok());
        assert!(task.is_completed());
    }

    #[test]
    fn test_task_reminders() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        let reminder = Reminder::new(
            "Test Reminder".to_string(),
            Utc::now() + Duration::from_secs(20),
            "description".to_string(),
        );
        task.add_reminder(reminder);
        assert_eq!(task.reminders().len(), 1);
    }

    #[test]
    fn test_task_reminder_overdue() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        let reminder = Reminder::new(
            "Test Reminder".to_string(),
            Utc::now() - Duration::from_secs(20),
            "description".to_string(),
        );
        task.add_reminder(reminder);
        assert_eq!(task.reminders().len(), 1);
        let overdue_reminders = task.overdue_reminders();
        assert_eq!(overdue_reminders.len(), 1);
    }

    #[test]
    fn test_task_reminder_overdue_sent() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        let reminder = Reminder::new(
            "Test Reminder".to_string(),
            Utc::now() - Duration::from_secs(20),
            "description".to_string(),
        );
        task.add_reminder(reminder);
        assert_eq!(task.reminders().len(), 1);
        let mut overdue_reminders = task.overdue_reminders_mut();
        assert_eq!(overdue_reminders.len(), 1);
        overdue_reminders[0].mark_sent();
        let overdue_reminders = task.overdue_reminders();
        assert_eq!(overdue_reminders.len(), 0);
    }

    #[test]
    fn test_task_priority() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        assert_eq!(task.priority(), None);
        task.set_priority(TaskPriority::High);
        assert_eq!(task.priority(), Some(TaskPriority::High));
        task.set_priority(TaskPriority::Low);
        assert_eq!(task.priority(), Some(TaskPriority::Low));
    }

    #[test]
    fn test_task_due_date() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        assert_eq!(task.due_date(), None);
        let due_date = Utc::now() + Duration::from_secs(30);
        task.set_due_date(Some(due_date));
        assert_eq!(task.due_date(), Some(due_date));
        let new_due_date = Utc::now() + Duration::from_secs(60);
        task.set_due_date(Some(new_due_date));
        assert_eq!(task.due_date(), Some(new_due_date));
    }

    #[test]
    fn test_task_overdue() {
        let user_id = Uuid::new_v4();
        let mut task = Task::new(user_id, "Test Task".to_string(), None, None);
        assert_eq!(task.due_date(), None);
        let due_date = Utc::now() - Duration::from_secs(30);
        task.set_due_date(Some(due_date));
        assert!(task.is_overdue())
    }
}
