use chrono::{DateTime, Duration, Utc};
use domain::tasks::entities::task_priority::TaskPriority;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use application::tasks::use_cases::task::{
    common::task_schedule_app_dto::TaskScheduleAppDto,
    get_tasks::{ReminderOutput, TaskOutput},
};

use crate::http_error::HttpError;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct TaskDto {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TaskPriorityDto>,
    pub schedule: TaskScheduleDto,
    #[cfg_attr(feature = "ts", ts(type = "number | null"))]
    pub completed_at: Option<i64>,
    pub subtasks: Vec<SubtaskDto>,
    #[cfg_attr(feature = "ts", ts(type = "string | null"))]
    pub category_id: Option<Uuid>,
    pub reminders: Vec<ReminderDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct ReminderDto {
    pub id: String,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub date_time: i64,
    pub title: String,
    pub description: String,
    pub reminder_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct SubtaskDto {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub sort_order: i16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export, rename = "TaskPriority"))]
#[serde(rename_all = "camelCase")]
pub enum TaskPriorityDto {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TaskScheduleDto {
    Unscheduled,
    AllDay {
        #[cfg_attr(feature = "ts", ts(type = "number"))]
        date: i64,
    },
    #[serde(rename_all = "camelCase")]
    At {
        #[cfg_attr(feature = "ts", ts(type = "number"))]
        starts_at: i64,
    },
    #[serde(rename_all = "camelCase")]
    Span {
        #[cfg_attr(feature = "ts", ts(type = "number"))]
        starts_at: i64,
        #[cfg_attr(feature = "ts", ts(type = "number"))]
        duration: i64,
    },
}

pub fn task_schedule_dto_to_app_dto(
    schedule: TaskScheduleDto,
) -> Result<TaskScheduleAppDto, HttpError> {
    match schedule {
        TaskScheduleDto::Unscheduled => Ok(TaskScheduleAppDto::Unscheduled),
        TaskScheduleDto::AllDay { date: timestamp } => {
            let date = DateTime::from_timestamp(timestamp, 0)
                .map(|dt| dt.date_naive())
                .ok_or(HttpError::BadRequest("Invalid date".to_string()))?;
            Ok(TaskScheduleAppDto::AllDay { date })
        }
        TaskScheduleDto::At { starts_at } => {
            let starts_at = DateTime::from_timestamp(starts_at, 0)
                .ok_or(HttpError::BadRequest("Invalid starts_at".to_string()))?;
            Ok(TaskScheduleAppDto::At { starts_at })
        }
        TaskScheduleDto::Span {
            starts_at,
            duration,
        } => {
            let starts_at = DateTime::from_timestamp(starts_at, 0)
                .ok_or(HttpError::BadRequest("Invalid starts_at".to_string()))?;
            let duration = Duration::seconds(duration);
            Ok(TaskScheduleAppDto::Span {
                starts_at,
                duration,
            })
        }
    }
}

pub fn task_schedule_app_dto_to_dto(schedule: TaskScheduleAppDto) -> TaskScheduleDto {
    match schedule {
        TaskScheduleAppDto::Unscheduled => TaskScheduleDto::Unscheduled,
        TaskScheduleAppDto::AllDay { date } => {
            let timestamp = date
                .and_hms_opt(0, 0, 0)
                .and_then(|naive_datetime| naive_datetime.and_local_timezone(Utc).single())
                .map(|dt| dt.timestamp())
                .unwrap_or(0);
            TaskScheduleDto::AllDay { date: timestamp }
        }
        TaskScheduleAppDto::At { starts_at } => TaskScheduleDto::At {
            starts_at: starts_at.timestamp(),
        },
        TaskScheduleAppDto::Span {
            starts_at,
            duration,
        } => TaskScheduleDto::Span {
            starts_at: starts_at.timestamp(),
            duration: duration.num_seconds(),
        },
    }
}

pub fn priority_to_dto(p: Option<TaskPriority>) -> Option<TaskPriorityDto> {
    p.map(|p| match p {
        TaskPriority::Low => TaskPriorityDto::Low,
        TaskPriority::Medium => TaskPriorityDto::Medium,
        TaskPriority::High => TaskPriorityDto::High,
        TaskPriority::Urgent => TaskPriorityDto::Urgent,
    })
}

pub fn reminder_to_dto(r: &ReminderOutput) -> ReminderDto {
    ReminderDto {
        id: r.id.to_string(),
        date_time: r.date_time.timestamp(),
        title: r.title.clone(),
        description: r.description.clone(),
        reminder_sent: r.reminder_sent,
    }
}

pub fn from_task_output(v: &TaskOutput) -> TaskDto {
    TaskDto {
        id: v.id.to_string(),
        title: v.title.clone(),
        description: v.description.clone(),
        priority: priority_to_dto(v.priority),
        schedule: task_schedule_app_dto_to_dto(v.schedule.clone()),
        completed_at: v.completed_at.map(|c| c.timestamp()),
        subtasks: v
            .subtasks
            .iter()
            .map(|s| SubtaskDto {
                id: s.id.to_string(),
                title: s.title.clone(),
                description: s.description.clone(),
                is_completed: s.is_completed,
                sort_order: s.sort_order,
            })
            .collect(),
        category_id: v.category_id,
        reminders: v.reminders.iter().map(reminder_to_dto).collect(),
    }
}
