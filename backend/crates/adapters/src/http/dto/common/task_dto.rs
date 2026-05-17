use chrono::{DateTime, Duration, Utc};
pub use shared::task::TaskDto;

use application::use_cases::task::{
    common::task_schedule_app_dto::TaskScheduleAppDto, get_tasks::TaskOutput,
};
use domain::entities::tasks::task_priority::TaskPriority;
use shared::task::{SubtaskDto, TaskPriority as SharedTaskPriority, TaskScheduleDto};

use crate::http_error::HttpError;

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

fn priority_to_dto(p: Option<TaskPriority>) -> Option<SharedTaskPriority> {
    p.map(|p| match p {
        TaskPriority::Low => SharedTaskPriority::Low,
        TaskPriority::Medium => SharedTaskPriority::Medium,
        TaskPriority::High => SharedTaskPriority::High,
        TaskPriority::Urgent => SharedTaskPriority::Urgent,
    })
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
    }
}
