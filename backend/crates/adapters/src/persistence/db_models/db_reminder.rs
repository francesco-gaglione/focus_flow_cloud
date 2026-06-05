use crate::persistence::schema;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use domain::tasks::entities::reminder::Reminder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::reminders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbReminder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub reminder_sent: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::reminders)]
pub struct NewDbReminder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub task_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub reminder_sent: bool,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::reminders)]
pub struct UpdateDbReminder {
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub reminder_sent: Option<bool>,
}

impl From<DbReminder> for Reminder {
    fn from(db: DbReminder) -> Self {
        Reminder::reconstitute(
            db.id,
            db.task_id,
            db.user_id,
            db.title,
            db.description,
            db.date,
            db.reminder_sent,
        )
    }
}

impl From<Reminder> for NewDbReminder {
    fn from(r: Reminder) -> Self {
        Self {
            id: r.id(),
            task_id: r.task_id(),
            user_id: r.user_id(),
            title: r.title().to_string(),
            description: r.description().to_string(),
            date: r.date(),
            reminder_sent: r.is_sent(),
        }
    }
}
