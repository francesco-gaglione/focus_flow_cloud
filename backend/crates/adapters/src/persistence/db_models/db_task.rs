use crate::persistence::schema;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use domain::entities::tasks::{subtask::Subtask, task::Task, task_priority::TaskPriority};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbTask {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub priority: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
pub struct NewDbTask {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub priority: Option<String>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
pub struct UpdateDbTask {
    pub category_id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub scheduled_date: Option<DateTime<Utc>>,
    #[diesel(treat_none_as_null = true)]
    pub completed_at: Option<DateTime<Utc>>,
    pub priority: Option<String>,
}

fn priority_to_str(p: TaskPriority) -> String {
    match p {
        TaskPriority::Low => "low".to_string(),
        TaskPriority::Medium => "medium".to_string(),
        TaskPriority::High => "high".to_string(),
        TaskPriority::Urgent => "urgent".to_string(),
    }
}

fn str_to_priority(s: &str) -> Option<TaskPriority> {
    match s {
        "low" => Some(TaskPriority::Low),
        "medium" => Some(TaskPriority::Medium),
        "high" => Some(TaskPriority::High),
        "urgent" => Some(TaskPriority::Urgent),
        _ => None,
    }
}

impl From<Task> for NewDbTask {
    fn from(value: Task) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            category_id: value.category_id(),
            title: value.title().to_string(),
            description: value.description().map(|s| s.to_string()),
            scheduled_date: value.due_date(),
            priority: value.priority().map(priority_to_str),
        }
    }
}

impl From<Task> for DbTask {
    fn from(value: Task) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            category_id: value.category_id(),
            title: value.title().to_string(),
            description: value.description().map(|s| s.to_string()),
            scheduled_date: value.due_date(),
            created_at: Default::default(),
            completed_at: value.completed_at(),
            priority: value.priority().map(priority_to_str),
        }
    }
}

impl From<Task> for UpdateDbTask {
    fn from(value: Task) -> Self {
        Self {
            category_id: value.category_id(),
            title: Some(value.title().to_string()),
            description: value.description().map(|s| s.to_string()),
            scheduled_date: value.due_date(),
            completed_at: value.completed_at(),
            priority: value.priority().map(priority_to_str),
        }
    }
}

impl From<DbTask> for Task {
    fn from(value: DbTask) -> Self {
        let priority = value.priority.as_deref().and_then(str_to_priority);
        Task::reconstitute(
            value.id,
            value.user_id,
            value.title,
            value.description,
            priority,
            value.category_id,
            vec![],
            value.scheduled_date,
            value.completed_at,
            vec![],
        )
    }
}

impl DbTask {
    pub fn into_task_with_subtasks(self, subtasks: Vec<Subtask>) -> Task {
        let priority = self.priority.as_deref().and_then(str_to_priority);
        Task::reconstitute(
            self.id,
            self.user_id,
            self.title,
            self.description,
            priority,
            self.category_id,
            subtasks,
            self.scheduled_date,
            self.completed_at,
            vec![],
        )
    }
}
