use crate::persistence::schema;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use domain::entities::tasks::task::Task;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbTask {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub scheduled_end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
pub struct NewDbTask {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub scheduled_end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
pub struct UpdateDbTask {
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub scheduled_end_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<Task> for NewDbTask {
    fn from(value: Task) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            category_id: None,
            name: value.title().to_string(),
            description: value.description().map(|s| s.to_string()),
            scheduled_date: value.due_date(),
            scheduled_end_date: None,
        }
    }
}

impl From<Task> for DbTask {
    fn from(value: Task) -> Self {
        Self {
            id: value.id(),
            user_id: value.user_id(),
            category_id: None,
            name: value.title().to_string(),
            description: value.description().map(|s| s.to_string()),
            scheduled_date: value.due_date(),
            scheduled_end_date: None,
            created_at: Default::default(),
            completed_at: value.completed_at(),
            deleted_at: None,
        }
    }
}

impl From<Task> for UpdateDbTask {
    fn from(value: Task) -> Self {
        Self {
            category_id: None,
            name: Some(value.title().to_string()),
            description: value.description().map(|s| s.to_string()),
            scheduled_date: value.due_date(),
            scheduled_end_date: None,
            completed_at: value.completed_at(),
        }
    }
}

impl From<DbTask> for Task {
    fn from(value: DbTask) -> Self {
        Task::reconstitute(
            value.id,
            value.user_id,
            value.name,
            value.description,
            None,
            vec![],
            value.scheduled_date,
            value.completed_at,
            vec![],
        )
    }
}
