use crate::repository_traits::persistence_error::PersistenceError;
use crate::repository_traits::reminder_persistence::ReminderPersistence;
use crate::repository_traits::task_persistence::TaskPersistence;
use crate::use_cases::task::common::task_schedule_app_dto::TaskScheduleAppDto;
use chrono::{DateTime, Utc};
use domain::tasks::entities::reminder::Reminder;
use domain::tasks::entities::subtask::Subtask;
use domain::tasks::entities::task::Task;
use domain::tasks::entities::task_priority::TaskPriority;
use domain::tasks::entities::task_schedule::TaskSchedule;
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
    pub completed: bool,
    pub today: bool,
    pub next_week: bool,
    pub unscheduled: bool,
    pub incoming: bool,
}

#[derive(Debug, Clone)]
pub struct TasksOutput {
    pub today: Vec<TaskOutput>,
    pub next_week: Vec<TaskOutput>,
    pub incoming: Vec<TaskOutput>,
    pub unscheduled: Vec<TaskOutput>,
    pub completed: Vec<TaskOutput>,
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
    pub async fn execute(&self, command: GetTasksCommand) -> GetTasksResult<TasksOutput> {
        tracing::info!("Fetching tasks with completed: {:?}", command.completed);
        let res = self.task_persistence.find_all().await?;
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

        // Filter and map tasks by schedule
        let mut today_tasks: Vec<TaskOutput> = if command.today {
            res.iter()
                .filter(|t| t.schedule().is_today() && !t.is_completed())
                .map(|t| {
                    let mut output = TaskOutput::from(t);
                    output.reminders = reminders_by_task.remove(&t.id()).unwrap_or_default();
                    output
                })
                .collect()
        } else {
            Vec::new()
        };

        let mut next_week_tasks: Vec<TaskOutput> = if command.next_week {
            res.iter()
                .filter(|t| t.schedule().is_next_week() && !t.is_completed())
                .map(|t| {
                    let mut output = TaskOutput::from(t);
                    output.reminders = reminders_by_task.remove(&t.id()).unwrap_or_default();
                    output
                })
                .collect()
        } else {
            Vec::new()
        };

        let mut incoming_tasks: Vec<TaskOutput> = if command.incoming {
            res.iter()
                .filter(|t| t.schedule().is_incoming() && !t.is_completed())
                .map(|t| {
                    let mut output = TaskOutput::from(t);
                    output.reminders = reminders_by_task.remove(&t.id()).unwrap_or_default();
                    output
                })
                .collect()
        } else {
            Vec::new()
        };

        let mut unscheduled_tasks: Vec<TaskOutput> = if command.unscheduled {
            res.iter()
                .filter(|t| t.schedule() == TaskSchedule::Unscheduled && !t.is_completed())
                .map(TaskOutput::from)
                .collect::<Vec<TaskOutput>>()
        } else {
            Vec::new()
        };

        let mut completed = if command.completed {
            res.iter()
                .filter(|t| t.is_completed())
                .map(TaskOutput::from)
                .collect::<Vec<TaskOutput>>()
        } else {
            Vec::new()
        };

        // Sort tasks by priority
        today_tasks.sort_by_key(|t| match t.priority {
            Some(TaskPriority::Urgent) => 0,
            Some(TaskPriority::High) => 1,
            Some(TaskPriority::Medium) => 2,
            Some(TaskPriority::Low) => 3,
            None => 4,
        });

        next_week_tasks.sort_by_key(|t| match t.priority {
            Some(TaskPriority::Urgent) => 0,
            Some(TaskPriority::High) => 1,
            Some(TaskPriority::Medium) => 2,
            Some(TaskPriority::Low) => 3,
            None => 4,
        });

        incoming_tasks.sort_by_key(|t| match t.priority {
            Some(TaskPriority::Urgent) => 0,
            Some(TaskPriority::High) => 1,
            Some(TaskPriority::Medium) => 2,
            Some(TaskPriority::Low) => 3,
            None => 4,
        });

        unscheduled_tasks.sort_by_key(|t| match t.priority {
            Some(TaskPriority::Urgent) => 0,
            Some(TaskPriority::High) => 1,
            Some(TaskPriority::Medium) => 2,
            Some(TaskPriority::Low) => 3,
            None => 4,
        });

        completed.sort_by_key(|t| match t.priority {
            Some(TaskPriority::Urgent) => 0,
            Some(TaskPriority::High) => 1,
            Some(TaskPriority::Medium) => 2,
            Some(TaskPriority::Low) => 3,
            None => 4,
        });

        Ok(TasksOutput {
            today: today_tasks,
            next_week: next_week_tasks,
            incoming: incoming_tasks,
            unscheduled: unscheduled_tasks,
            completed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::reminder_persistence::MockReminderPersistence;
    use crate::repository_traits::task_persistence::MockTaskPersistence;
    use chrono::{Duration, Utc};
    use domain::tasks::entities::task_schedule::TaskSchedule;

    fn create_dummy_task(title: &str, schedule: TaskSchedule) -> Task {
        Task::new(Uuid::new_v4(), title.to_string(), schedule, None)
    }

    fn make_use_case(
        mock_task: MockTaskPersistence,
        mock_reminder: MockReminderPersistence,
    ) -> GetTasksUseCase {
        GetTasksUseCase::new(Arc::new(mock_task), Arc::new(mock_reminder))
    }

    #[tokio::test]
    async fn test_get_tasks_success_bucketing_and_sorting() {
        let mut mock_persistence = MockTaskPersistence::new();
        let mut mock_reminder = MockReminderPersistence::new();

        let now = Utc::now();

        let task_today = create_dummy_task(
            "Today Task",
            TaskSchedule::AllDay {
                date: now.date_naive(),
            },
        );

        let task_next_week = create_dummy_task(
            "Next Week Task",
            TaskSchedule::AllDay {
                date: now.date_naive() + Duration::days(2),
            },
        );

        let task_incoming = create_dummy_task(
            "Incoming Task",
            TaskSchedule::AllDay {
                date: now.date_naive() + Duration::days(10),
            },
        );

        let task_unscheduled = create_dummy_task("Unscheduled Task", TaskSchedule::Unscheduled);

        let returned_tasks = vec![
            task_today.clone(),
            task_next_week.clone(),
            task_incoming.clone(),
            task_unscheduled.clone(),
        ];

        mock_persistence
            .expect_find_all()
            .returning(move || Ok(returned_tasks.clone()));

        mock_reminder
            .expect_find_by_task_ids()
            .returning(|_| Ok(vec![]));

        let use_case = make_use_case(mock_persistence, mock_reminder);

        let command = GetTasksCommand {
            completed: false,
            today: true,
            next_week: true,
            incoming: true,
            unscheduled: true,
        };

        let result = use_case
            .execute(command)
            .await
            .expect("Execution should not fail");

        assert_eq!(result.today.len(), 1);
        assert_eq!(result.today[0].title, "Today Task");

        assert_eq!(result.next_week.len(), 1);
        assert_eq!(result.next_week[0].title, "Next Week Task");

        assert_eq!(result.incoming.len(), 1);
        assert_eq!(result.incoming[0].title, "Incoming Task");

        assert_eq!(result.completed.len(), 0);
    }

    #[tokio::test]
    async fn test_get_tasks_completed() {
        let mut mock_persistence = MockTaskPersistence::new();
        let mut mock_reminder = MockReminderPersistence::new();

        mock_persistence
            .expect_find_all()
            .returning(move || Ok(vec![]));

        mock_reminder
            .expect_find_by_task_ids()
            .returning(|_| Ok(vec![]));

        let use_case = make_use_case(mock_persistence, mock_reminder);

        let command = GetTasksCommand {
            completed: true,
            today: false,
            next_week: false,
            incoming: false,
            unscheduled: false,
        };

        let result = use_case
            .execute(command)
            .await
            .expect("Execution should not fail");

        assert_eq!(result.today.len(), 0);
        assert_eq!(result.next_week.len(), 0);
        assert_eq!(result.incoming.len(), 0);
        assert_eq!(result.completed.len(), 0);
    }
}
