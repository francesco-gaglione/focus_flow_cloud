use chrono::{DateTime, Utc};
use shared::task::{CreateSubtaskDto, CreateTaskDto};
use uuid::Uuid;

use crate::clients::{http_client::ApiError, task_http_client::create_task};

#[derive(Debug, thiserror::Error)]
pub enum CreateTaskError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Invalid category")]
    InvalidCategory,

    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub type CreateTaskResult<T> = Result<T, CreateTaskError>;

pub struct CreateTaskCommand {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub category_id: Option<String>,
    pub subtasks: Vec<CreateSubtask>,
}

pub struct CreateSubtask {
    pub title: String,
    pub description: Option<String>,
}

pub async fn create_task_uc(command: CreateTaskCommand) -> CreateTaskResult<()> {
    let category_id = command
        .category_id
        .map(|c| Uuid::parse_str(&c))
        .transpose()
        .map_err(|_| CreateTaskError::InvalidCategory)?;

    let subtasks = if !command.subtasks.is_empty() {
        Some(
            command
                .subtasks
                .iter()
                .map(|s| CreateSubtaskDto {
                    title: s.title.clone(),
                    description: s.description.clone(),
                })
                .collect(),
        )
    } else {
        None
    };

    let dto = CreateTaskDto {
        title: command.title,
        description: command.description,
        due_date: command.due_date.map(|d| d.timestamp()),
        subtasks,
        category_id,
    };

    let _ = create_task(dto).await?;

    Ok(())
}
