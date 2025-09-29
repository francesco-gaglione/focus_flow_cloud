use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::session_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionEvent {
    pub id: Uuid,
    pub session_id: Uuid,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::session_events)]
pub struct NewSessionEvent {
    pub session_id: Uuid,
    pub event_type: String,
    pub notes: Option<String>,
}
