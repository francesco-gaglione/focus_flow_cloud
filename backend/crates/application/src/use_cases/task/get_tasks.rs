use crate::repository_traits::persistence_error::PersistenceError;
use crate::repository_traits::reminder_persistence::ReminderPersistence;
use crate::repository_traits::task_persistence::TaskPersistence;
use crate::use_cases::task::common::task_schedule_app_dto::TaskScheduleAppDto;
use chrono::{DateTime, Utc};
use domain::entities::reminder::Reminder;
use domain::entities::tasks::subtask::Subtask;
use domain::entities::tasks::task::Task;
use domain::entities::tasks::task_priority::TaskPriority;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum GetTaskError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetTasksResult<T> = Result<T, GetTaskError>;

#[derive(Debug)]
pub struct GetTasksCommand {
    pub completed: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TaskOutput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub schedule: TaskScheduleAppDto,
    pub completed_at: Option<DateTime<Utc>>,
    pub subtasks: Vec<SubTaskOutput>,
    pub category_id: Option<Uuid>,
    pub reminders: Vec<ReminderOutput>,
}

#[derive(Debug, Clone)]
pub struct SubTaskOutput {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub sort_order: i16,
}

#[derive(Debug, Clone)]
pub struct ReminderOutput {
    pub id: Uuid,
    pub date_time: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub reminder_sent: bool,
}

impl From<&Task> for TaskOutput {
    fn from(value: &Task) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            title: value.title().to_string(),
            description: value.description().map(|d| d.to_string()),
            priority: value.priority(),
            schedule: value.schedule().into(),
            completed_at: value.completed_at(),
            subtasks: value.sub_tasks().iter().map(|s| s.into()).collect(),
            category_id: value.category_id(),
            reminders: vec![],
        }
    }
}

impl From<&Subtask> for SubTaskOutput {
    fn from(value: &Subtask) -> Self {
        Self {
            id: value.id(),
            title: value.title().to_string(),
            description: value.description().map(|d| d.to_string()),
            is_completed: value.is_completed(),
            sort_order: value.sort_order(),
        }
    }
}

impl From<&Reminder> for ReminderOutput {
    fn from(value: &Reminder) -> Self {
        Self {
            id: value.id(),
            date_time: value.date(),
            title: value.title().to_string(),
            description: value.description().to_string(),
            reminder_sent: value.is_sent(),
        }
    }
}

pub struct GetTasksUseCase {
    task_persistence: Arc<dyn TaskPersistence>,
    reminder_persistence: Arc<dyn ReminderPersistence>,
}

impl GetTasksUseCase {
    pub fn new(
        task_persistence: Arc<dyn TaskPersistence>,
        reminder_persistence: Arc<dyn ReminderPersistence>,
    ) -> Self {
        Self {
            task_persistence,
            reminder_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: GetTasksCommand) -> GetTasksResult<Vec<TaskOutput>> {
        tracing::info!("Fetching tasks with completed: {:?}", command.completed);
        let res = self.task_persistence.find_all(command.completed).await?;
        tracing::info!("Fetched {} tasks", res.len());

        let task_ids: Vec<Uuid> = res.iter().map(|t| t.id()).collect();
        let all_reminders = self.reminder_persistence.find_by_task_ids(task_ids).await?;

        let mut reminders_by_task: HashMap<Uuid, Vec<ReminderOutput>> = HashMap::new();
        for reminder in &all_reminders {
            if let Some(task_id) = reminder.task_id() {
                reminders_by_task
                    .entry(task_id)
                    .or_default()
                    .push(reminder.into());
            }
        }

        let mut tasks: Vec<TaskOutput> = res
            .iter()
            .map(|t| {
                let mut output = TaskOutput::from(t);
                output.reminders = reminders_by_task.remove(&t.id()).unwrap_or_default();
                output
            })
            .collect();

        tasks.sort_by_key(|t| match t.priority {
            Some(TaskPriority::Urgent) => 0,
            Some(TaskPriority::High) => 1,
            Some(TaskPriority::Medium) => 2,
            Some(TaskPriority::Low) => 3,
            None => 4,
        });
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use domain::entities::tasks::task_schedule::TaskSchedule;

    use super::*;
    use crate::repository_traits::reminder_persistence::MockReminderPersistence;
    use crate::repository_traits::task_persistence::MockTaskPersistence;

    fn make_use_case(
        mock_task: MockTaskPersistence,
        mock_reminder: MockReminderPersistence,
    ) -> GetTasksUseCase {
        GetTasksUseCase::new(Arc::new(mock_task), Arc::new(mock_reminder))
    }

    #[tokio::test]
    async fn test_get_tasks_success() {
        let mut mock_persistence = MockTaskPersistence::new();
        let mut mock_reminder = MockReminderPersistence::new();

        let expected_tasks = vec![Task::new(
            Uuid::new_v4(),
            "Task 1".to_string(),
            TaskSchedule::Unscheduled,
            None,
        )];
        let returned_tasks = expected_tasks.clone();

        mock_persistence
            .expect_find_all()
            .with(mockall::predicate::eq(None))
            .returning(move |_| Ok(returned_tasks.clone()));

        mock_reminder
            .expect_find_by_task_ids()
            .returning(|_| Ok(vec![]));

        let use_case = make_use_case(mock_persistence, mock_reminder);
        let command = GetTasksCommand { completed: None };
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_tasks_completed() {
        let mut mock_persistence = MockTaskPersistence::new();
        let mut mock_reminder = MockReminderPersistence::new();

        mock_persistence
            .expect_find_all()
            .with(mockall::predicate::eq(Some(true)))
            .returning(move |_| Ok(vec![]));

        mock_reminder
            .expect_find_by_task_ids()
            .returning(|_| Ok(vec![]));

        let use_case = make_use_case(mock_persistence, mock_reminder);
        let command = GetTasksCommand {
            completed: Some(true),
        };
        let result = use_case.execute(command).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
