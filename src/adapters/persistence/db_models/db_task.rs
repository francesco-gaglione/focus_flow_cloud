use crate::adapters::schema;
use crate::application::use_cases::persistance_command::create_task_data::CreateTaskData;
use crate::application::use_cases::persistance_command::update_task_data::UpdateTaskData;
use crate::domain::entities::task::Task;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl From<CreateTaskData> for NewDbTask {
    fn from(value: CreateTaskData) -> Self {
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
            id: value.id(),
            category_id: value.category_id(),
            name: value.name().to_string(),
            description: value.description().map(|s| s.to_string()),
            scheduled_date: value.scheduled_date(),
            created_at: Default::default(),
            completed_at: value.completed_at(),
            deleted_at: None,
        }
    }
}

impl From<Task> for UpdateDbTask {
    fn from(value: Task) -> Self {
        Self {
            category_id: value.category_id(),
            name: Some(value.name().to_string()),
            description: value.description().map(|s| s.to_string()),
            scheduled_date: value.scheduled_date(),
            completed_at: value.completed_at(),
        }
    }
}

impl From<DbTask> for Task {
    fn from(value: DbTask) -> Self {
        Self::new(
            value.id,
            value.category_id,
            value.name,
            value.description,
            value.scheduled_date,
            value.completed_at,
        )
    }
}

impl From<UpdateTaskData> for UpdateDbTask {
    fn from(value: UpdateTaskData) -> Self {
        Self {
            category_id: value.category_id,
            name: value.name,
            description: value.description,
            scheduled_date: value.scheduled_date,
            completed_at: value.completed_at,
        }
    }
}
