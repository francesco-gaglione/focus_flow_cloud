use chrono::{DateTime, Duration, NaiveDate, Utc};
use domain::entities::tasks::task_schedule::TaskSchedule;

#[derive(Debug, Clone)]
pub enum TaskScheduleAppDto {
    Unscheduled,
    AllDay {
        date: NaiveDate,
    },
    At {
        starts_at: DateTime<Utc>,
    },
    Span {
        starts_at: DateTime<Utc>,
        duration: Duration,
    },
}

impl Default for TaskScheduleAppDto {
    fn default() -> Self {
        Self::Unscheduled
    }
}

impl From<TaskSchedule> for TaskScheduleAppDto {
    fn from(value: TaskSchedule) -> Self {
        match value {
            TaskSchedule::Unscheduled => TaskScheduleAppDto::Unscheduled,
            TaskSchedule::AllDay { date } => TaskScheduleAppDto::AllDay { date },
            TaskSchedule::At { starts_at } => TaskScheduleAppDto::At { starts_at },
            TaskSchedule::Span {
                starts_at,
                duration,
            } => TaskScheduleAppDto::Span {
                starts_at,
                duration,
            },
        }
    }
}

impl Into<TaskSchedule> for TaskScheduleAppDto {
    fn into(self) -> TaskSchedule {
        match self {
            TaskScheduleAppDto::Unscheduled => TaskSchedule::Unscheduled,
            TaskScheduleAppDto::AllDay { date } => TaskSchedule::AllDay { date },
            TaskScheduleAppDto::At { starts_at } => TaskSchedule::At { starts_at },
            TaskScheduleAppDto::Span {
                starts_at,
                duration,
            } => TaskSchedule::Span {
                starts_at,
                duration,
            },
        }
    }
}
