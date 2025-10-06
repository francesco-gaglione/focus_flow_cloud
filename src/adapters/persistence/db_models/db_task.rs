use crate::application::use_cases::commands::create_task::CreateTaskCommand;
use crate::application::use_cases::commands::update_task::UpdateTaskCommand;
use crate::domain::entities::task::Task;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::adapters::schema;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbTask {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
pub struct NewDbTask {
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
pub struct UpdateDbTask {
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<CreateTaskCommand> for NewDbTask {
    fn from(value: CreateTaskCommand) -> Self {
        Self {
            category_id: value.category_id,
            name: value.name,
            description: value.description,
            scheduled_date: value.scheduled_date,
        }
    }
}

impl From<Task> for DbTask {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            name: value.name,
            description: value.description,
            scheduled_date: value.scheduled_date,
            created_at: Default::default(),
            completed_at: value.completed_at,
            deleted_at: None,
        }
    }
}

impl From<Task> for UpdateDbTask {
    fn from(value: Task) -> Self {
        Self {
            category_id: value.category_id,
            name: Some(value.name),
            description: value.description,
            scheduled_date: value.scheduled_date,
            completed_at: value.completed_at,
        }
    }
}

impl From<DbTask> for Task {
    fn from(value: DbTask) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            name: value.name,
            description: value.description,
            scheduled_date: value.scheduled_date,
            completed_at: value.completed_at,
        }
    }
}

impl From<UpdateTaskCommand> for UpdateDbTask {
    fn from(value: UpdateTaskCommand) -> Self {
        Self {
            category_id: value.category_id,
            name: value.name,
            description: value.description,
            scheduled_date: value.scheduled_date,
            completed_at: value.completed_at,
        }
    }
}
