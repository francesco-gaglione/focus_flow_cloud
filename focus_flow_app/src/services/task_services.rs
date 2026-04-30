use chrono::{DateTime, Local, Utc};
use dioxus::signals::{ReadableExt, Signal};
use shared::task::{TaskDto, TasksResponseDto};

use crate::{
    model::task::{Task, TaskDue, TaskPriority},
    services::api_client::{ApiClient, ApiError},
};

#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("{0}")]
    Generic(String),
}

impl From<ApiError> for TaskError {
    fn from(e: ApiError) -> Self {
        match e {
            ApiError::Network(msg) => TaskError::Network(msg),
            ApiError::InternalServerError(msg) => TaskError::Server(msg),
            ApiError::BadRequest(msg) => TaskError::Generic(msg),
            ApiError::Unauthorized => TaskError::Unauthorized,
            ApiError::Deserialization(msg) => TaskError::Deserialization(msg),
            ApiError::Unknown(msg) => TaskError::Generic(msg),
        }
    }
}

pub type TaskResult<T> = Result<T, TaskError>;

pub async fn get_all_tasks() -> TaskResult<Vec<Task>> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| TaskError::Generic("ApiClient signal not found".to_string()))?;

    let api = api_signal.read();

    let response: TasksResponseDto = api
        .get("/api/task", None, None)
        .await
        .map_err(TaskError::from)?;

    Ok(response.tasks.into_iter().map(Task::from).collect())
}

impl From<TaskDto> for Task {
    fn from(dto: TaskDto) -> Self {
        let today = Local::now().date_naive();
        let tomorrow = today.succ_opt().unwrap_or(today);

        let due = match dto.due_date {
            Some(ts) => {
                let date = DateTime::from_timestamp(ts, 0)
                    .map(|dt: DateTime<Utc>| dt.with_timezone(&Local).date_naive())
                    .unwrap_or(today);
                if date < today {
                    TaskDue::Overdue(date)
                } else if date == today {
                    TaskDue::Today
                } else if date == tomorrow {
                    TaskDue::Tomorrow
                } else {
                    TaskDue::Upcoming(date)
                }
            }
            None => TaskDue::Upcoming(today),
        };

        let priority = dto.priority.map(|p| match p {
            shared::task::TaskPriority::Low => TaskPriority::Low,
            shared::task::TaskPriority::Medium => TaskPriority::Medium,
            shared::task::TaskPriority::High => TaskPriority::High,
            shared::task::TaskPriority::Urgent => TaskPriority::Urgent,
        });

        let completed_at = dto.completed_at.and_then(|ts| {
            DateTime::from_timestamp(ts, 0)
                .map(|dt: DateTime<Utc>| dt.with_timezone(&Local).naive_local())
        });

        Task {
            id: dto.id,
            title: dto.title,
            description: dto.description,
            cat: String::new(),
            cat_color: String::new(),
            priority,
            due,
            completed_at,
            done: dto.completed_at.is_some(),
        }
    }
}
