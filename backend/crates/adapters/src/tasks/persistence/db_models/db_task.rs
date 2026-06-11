use crate::shared::persistence::schema;
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use domain::tasks::entities::{
    subtask::Subtask, task::Task, task_priority::TaskPriority, task_schedule::TaskSchedule,
};
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
    pub deleted_at: Option<DateTime<Utc>>,
    pub schedule_type: String,
    pub schedule_all_day_date: Option<NaiveDate>,
    pub schedule_duration_secs: Option<i64>,
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
    pub schedule_type: String,
    pub schedule_all_day_date: Option<NaiveDate>,
    pub schedule_duration_secs: Option<i64>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
pub struct UpdateDbTask {
    pub category_id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[diesel(treat_none_as_null = true)]
    pub scheduled_date: Option<DateTime<Utc>>,
    #[diesel(treat_none_as_null = true)]
    pub completed_at: Option<DateTime<Utc>>,
    pub priority: Option<String>,
    pub schedule_type: String,
    #[diesel(treat_none_as_null = true)]
    pub schedule_all_day_date: Option<NaiveDate>,
    #[diesel(treat_none_as_null = true)]
    pub schedule_duration_secs: Option<i64>,
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

// Returns (schedule_type, scheduled_date, schedule_all_day_date, schedule_duration_secs)
fn schedule_to_db(
    schedule: TaskSchedule,
) -> (
    String,
    Option<DateTime<Utc>>,
    Option<NaiveDate>,
    Option<i64>,
) {
    match schedule {
        TaskSchedule::Unscheduled => ("unscheduled".to_string(), None, None, None),
        TaskSchedule::AllDay { date } => {
            let dt = date.and_time(NaiveTime::MIN).and_utc();
            ("all_day".to_string(), Some(dt), Some(date), None)
        }
        TaskSchedule::At { starts_at } => ("at".to_string(), Some(starts_at), None, None),
        TaskSchedule::Span {
            starts_at,
            duration,
        } => (
            "span".to_string(),
            Some(starts_at),
            None,
            Some(duration.num_seconds()),
        ),
    }
}

fn schedule_from_db(
    schedule_type: &str,
    scheduled_date: Option<DateTime<Utc>>,
    schedule_all_day_date: Option<NaiveDate>,
    schedule_duration_secs: Option<i64>,
) -> TaskSchedule {
    match schedule_type {
        "all_day" => schedule_all_day_date
            .map(|date| TaskSchedule::AllDay { date })
            .unwrap_or(TaskSchedule::Unscheduled),
        "at" => scheduled_date
            .map(|starts_at| TaskSchedule::At { starts_at })
            .unwrap_or(TaskSchedule::Unscheduled),
        "span" => match (scheduled_date, schedule_duration_secs) {
            (Some(starts_at), Some(secs)) => TaskSchedule::Span {
                starts_at,
                duration: chrono::Duration::seconds(secs),
            },
            _ => TaskSchedule::Unscheduled,
        },
        _ => TaskSchedule::Unscheduled,
    }
}

impl From<Task> for NewDbTask {
    fn from(value: Task) -> Self {
        let (schedule_type, scheduled_date, schedule_all_day_date, schedule_duration_secs) =
            schedule_to_db(value.schedule());
        Self {
            id: value.id(),
            user_id: value.user_id(),
            category_id: value.category_id(),
            title: value.title().to_string(),
            description: value.description().map(|s| s.to_string()),
            scheduled_date,
            priority: value.priority().map(priority_to_str),
            schedule_type,
            schedule_all_day_date,
            schedule_duration_secs,
        }
    }
}

impl From<Task> for DbTask {
    fn from(value: Task) -> Self {
        let (schedule_type, scheduled_date, schedule_all_day_date, schedule_duration_secs) =
            schedule_to_db(value.schedule());
        Self {
            id: value.id(),
            user_id: value.user_id(),
            category_id: value.category_id(),
            title: value.title().to_string(),
            description: value.description().map(|s| s.to_string()),
            scheduled_date,
            created_at: Default::default(),
            completed_at: value.completed_at(),
            priority: value.priority().map(priority_to_str),
            deleted_at: None,
            schedule_type,
            schedule_all_day_date,
            schedule_duration_secs,
        }
    }
}

impl From<Task> for UpdateDbTask {
    fn from(value: Task) -> Self {
        let (schedule_type, scheduled_date, schedule_all_day_date, schedule_duration_secs) =
            schedule_to_db(value.schedule());
        Self {
            category_id: value.category_id(),
            title: Some(value.title().to_string()),
            description: value.description().map(|s| s.to_string()),
            scheduled_date,
            completed_at: value.completed_at(),
            priority: value.priority().map(priority_to_str),
            schedule_type,
            schedule_all_day_date,
            schedule_duration_secs,
        }
    }
}

impl From<DbTask> for Task {
    fn from(value: DbTask) -> Self {
        let priority = value.priority.as_deref().and_then(str_to_priority);
        let schedule = schedule_from_db(
            &value.schedule_type,
            value.scheduled_date,
            value.schedule_all_day_date,
            value.schedule_duration_secs,
        );
        Task::reconstitute(
            value.id,
            value.user_id,
            value.title,
            value.description,
            priority,
            value.category_id,
            vec![],
            schedule,
            value.completed_at,
            vec![],
        )
    }
}

impl DbTask {
    pub fn into_task_with_subtasks(self, subtasks: Vec<Subtask>) -> Task {
        let priority = self.priority.as_deref().and_then(str_to_priority);
        let schedule = schedule_from_db(
            &self.schedule_type,
            self.scheduled_date,
            self.schedule_all_day_date,
            self.schedule_duration_secs,
        );
        Task::reconstitute(
            self.id,
            self.user_id,
            self.title,
            self.description,
            priority,
            self.category_id,
            subtasks,
            schedule,
            self.completed_at,
            vec![],
        )
    }
}
