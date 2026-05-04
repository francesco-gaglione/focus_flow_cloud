use crate::persistence::schema;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use domain::entities::tasks::subtask::Subtask;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::subtasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbSubtask {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub sort_order: i16,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::subtasks)]
pub struct NewDbSubtask {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i16,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::subtasks)]
pub struct UpdateDbSubtask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub sort_order: Option<i16>,
}

impl NewDbSubtask {
    pub fn from_subtask(subtask: &Subtask, task_id: Uuid, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            task_id,
            user_id,
            title: subtask.title().to_string(),
            description: subtask.description().map(|s| s.to_string()),
            sort_order: subtask.sort_order(),
        }
    }
}

impl From<DbSubtask> for Subtask {
    fn from(value: DbSubtask) -> Self {
        Subtask::new(
            value.title,
            value.sort_order,
            value.description,
            Some(value.id),
        )
    }
}
