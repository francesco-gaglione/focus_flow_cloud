use chrono::{DateTime, Local, Timelike, Utc};
use dioxus::logger::tracing::debug;
use shared::task::{TaskPriority, TaskScheduleDto, TasksResponseDto};

use crate::clients::{
    category_http_client::get_all_categories, http_client::ApiError,
    task_http_client::get_all_tasks,
};

#[derive(Debug, thiserror::Error)]
pub enum TaskListError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type TaskListResult<T> = Result<T, TaskListError>;

pub struct TaskList {
    pub tasks: Vec<TodoTask>,
    pub categories: Vec<TodoCategory>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TodoCategory {
    pub name: String,
    pub id: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TodoSubtask {
    pub id: String,
    pub title: String,
    pub is_completed: bool,
    pub sort_order: i16,
}

/// App-layer schedule — parsed chrono types, no raw timestamps.
#[derive(Clone, PartialEq, Debug)]
pub enum TaskSchedule {
    Unscheduled,
    AllDay {
        date: chrono::NaiveDate,
    },
    At {
        starts_at: chrono::DateTime<Local>,
    },
    Span {
        starts_at: chrono::DateTime<Local>,
        duration_secs: i64,
    },
}

impl TaskSchedule {
    pub fn is_scheduled(&self) -> bool {
        !matches!(self, TaskSchedule::Unscheduled)
    }

    pub fn naive_date(&self) -> Option<chrono::NaiveDate> {
        match self {
            TaskSchedule::Unscheduled => None,
            TaskSchedule::AllDay { date } => Some(*date),
            TaskSchedule::At { starts_at } => Some(starts_at.date_naive()),
            TaskSchedule::Span { starts_at, .. } => Some(starts_at.date_naive()),
        }
    }

    pub fn time_str(&self) -> Option<String> {
        match self {
            TaskSchedule::At { starts_at } if starts_at.hour() != 0 || starts_at.minute() != 0 => {
                Some(starts_at.format("%H:%M").to_string())
            }
            TaskSchedule::Span { starts_at, .. } => Some(starts_at.format("%H:%M").to_string()),
            _ => None,
        }
    }

    pub fn is_all_day(&self) -> bool {
        matches!(self, TaskSchedule::AllDay { .. })
    }

    pub fn duration_label(&self) -> Option<String> {
        match self {
            TaskSchedule::Span { duration_secs, .. } => {
                let total_mins = duration_secs / 60;
                if total_mins == 0 {
                    return None;
                }
                let h = total_mins / 60;
                let m = total_mins % 60;
                Some(if h > 0 && m > 0 {
                    format!("{}h{}m", h, m)
                } else if h > 0 {
                    format!("{}h", h)
                } else {
                    format!("{}m", m)
                })
            }
            _ => None,
        }
    }

    pub fn duration_mins(&self) -> Option<i64> {
        match self {
            TaskSchedule::Span { duration_secs, .. } => Some(duration_secs / 60),
            _ => None,
        }
    }

    /// HH:MM of start + duration end, for pre-populating "end time" input.
    pub fn end_time_str(&self) -> Option<String> {
        match self {
            TaskSchedule::Span {
                starts_at,
                duration_secs,
            } => {
                let end = *starts_at + chrono::Duration::seconds(*duration_secs);
                Some(end.format("%H:%M").to_string())
            }
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct TodoTask {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub cat: Option<String>,
    pub cat_id: Option<String>,
    pub cat_color: Option<String>,
    pub priority: Option<TaskPriority>,
    pub schedule: TaskSchedule,
    pub due: TaskDue,
    pub due_date_set: bool,
    pub due_time: Option<String>,
    pub completed_at: Option<chrono::NaiveDateTime>,
    pub done: bool,
    pub subtasks: Vec<TodoSubtask>,
}

#[derive(Clone, PartialEq)]
pub enum TaskDue {
    Overdue(chrono::NaiveDate),
    Today(chrono::NaiveDate),
    Tomorrow(chrono::NaiveDate),
    Upcoming(chrono::NaiveDate),
}

impl std::fmt::Display for TaskDue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskDue::Overdue(date) => write!(f, "Overdue ({})", date.format("%b %d")),
            TaskDue::Today(_) => write!(f, "Today"),
            TaskDue::Tomorrow(_) => write!(f, "Tomorrow"),
            TaskDue::Upcoming(date) => write!(f, "{}", date.format("%b %d")),
        }
    }
}

fn parse_schedule(dto: TaskScheduleDto) -> TaskSchedule {
    match dto {
        TaskScheduleDto::Unscheduled => TaskSchedule::Unscheduled,
        TaskScheduleDto::AllDay { date } => {
            let naive = DateTime::from_timestamp(date, 0)
                .map(|dt: DateTime<Utc>| dt.date_naive())
                .unwrap_or_else(|| Local::now().date_naive());
            TaskSchedule::AllDay { date: naive }
        }
        TaskScheduleDto::At { starts_at } => {
            let dt = DateTime::from_timestamp(starts_at, 0)
                .map(|dt: DateTime<Utc>| dt.with_timezone(&Local))
                .unwrap_or_else(Local::now);
            TaskSchedule::At { starts_at: dt }
        }
        TaskScheduleDto::Span {
            starts_at,
            duration,
        } => {
            let dt = DateTime::from_timestamp(starts_at, 0)
                .map(|dt: DateTime<Utc>| dt.with_timezone(&Local))
                .unwrap_or_else(Local::now);
            TaskSchedule::Span {
                starts_at: dt,
                duration_secs: duration,
            }
        }
    }
}

fn categorize_due(
    date: chrono::NaiveDate,
    today: chrono::NaiveDate,
    tomorrow: chrono::NaiveDate,
) -> TaskDue {
    if date < today {
        TaskDue::Overdue(date)
    } else if date == today {
        TaskDue::Today(date)
    } else if date == tomorrow {
        TaskDue::Tomorrow(date)
    } else {
        TaskDue::Upcoming(date)
    }
}

pub async fn task_list_uc(_category_id: Option<&str>) -> TaskListResult<TaskList> {
    let tasks_dto: TasksResponseDto = get_all_tasks().await?;
    debug!("Tasks dto: {:?}", tasks_dto);
    let categories = get_all_categories().await?;

    let today = Local::now().date_naive();
    let tomorrow = today.succ_opt().unwrap_or(today);

    let mut todo_tasks: Vec<TodoTask> = vec![];

    for task in tasks_dto.tasks {
        let category = if let Some(cat_id) = task.category_id {
            categories
                .categories
                .iter()
                .find(|c| c.id == cat_id.to_string())
        } else {
            None
        };

        let schedule = parse_schedule(task.schedule);
        let due_date_set = schedule.is_scheduled();
        let due_time = schedule.time_str();
        let due = match schedule.naive_date() {
            Some(date) => categorize_due(date, today, tomorrow),
            None => TaskDue::Upcoming(today),
        };

        let subtasks = task
            .subtasks
            .into_iter()
            .map(|s| TodoSubtask {
                id: s.id,
                title: s.title,
                is_completed: s.is_completed,
                sort_order: s.sort_order,
            })
            .collect();

        todo_tasks.push(TodoTask {
            id: task.id,
            title: task.title,
            description: task.description,
            cat: category.map(|c| c.name.clone()),
            cat_id: task.category_id.map(|c| c.to_string()),
            cat_color: category.map(|c| c.color.clone()),
            priority: task.priority,
            schedule,
            due,
            due_date_set,
            due_time,
            completed_at: task.completed_at.and_then(|ts| {
                DateTime::from_timestamp(ts, 0)
                    .map(|dt: DateTime<Utc>| dt.with_timezone(&Local).naive_local())
            }),
            done: task.completed_at.is_some(),
            subtasks,
        });
    }

    Ok(TaskList {
        tasks: todo_tasks,
        categories: categories
            .categories
            .iter()
            .map(|c| TodoCategory {
                name: c.name.clone(),
                id: c.id.clone(),
            })
            .collect(),
    })
}
