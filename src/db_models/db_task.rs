use chrono::{DateTime, NaiveDate, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbTask {
    pub id: Uuid,
    pub template_id: Option<Uuid>,
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewDbTask {
    pub template_id: Option<Uuid>,
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct UpdateDbTask {
    pub template_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
}
