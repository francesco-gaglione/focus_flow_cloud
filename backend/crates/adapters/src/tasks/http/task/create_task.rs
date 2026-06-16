use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use crate::shared::http::app_state::AppState;
use crate::shared::http::model::session_model::UserSession;
use crate::tasks::http::dto::task_dto::task_schedule_dto_to_app_dto;
use crate::tasks::http::dto::task_dto::{TaskPriorityDto, TaskScheduleDto};
use application::tasks::use_cases::task::create_task::{
    AddReminderCommand, CreateSubtaskCommand, CreateTaskCommand, CreateTaskError,
};
use axum::{extract::State, Extension, Json};
use chrono::DateTime;
use domain::tasks::entities::task_priority::TaskPriority as DomainPriority;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CreateSubtaskDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,
    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CreateReminderDto {
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub date: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,
    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,
    pub schedule: Option<TaskScheduleDto>,
    #[validate(nested)]
    pub subtasks: Option<Vec<CreateSubtaskDto>>,
    #[cfg_attr(feature = "ts", ts(type = "string | null"))]
    pub category_id: Option<Uuid>,
    pub priority: Option<TaskPriorityDto>,
    pub reminders: Option<Vec<CreateReminderDto>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct CreateTaskResponseDto {
    pub id: String,
}

impl From<CreateTaskError> for HttpError {
    fn from(value: CreateTaskError) -> Self {
        match value {
            CreateTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
            CreateTaskError::WorkerPortError(e) => HttpError::GenericError(e.to_string()),
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/task",
    tag = TASK_TAG,
    summary = "Create a new task",
    request_body = CreateTaskDto,
    responses(
        (status = 201, description = "Task created successfully", body = CreateTaskResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn create_task_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Json(payload): Json<CreateTaskDto>,
) -> HttpResult<Json<CreateTaskResponseDto>> {
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;
    tracing::info!("Creating task with title");

    let schedule = payload
        .schedule
        .map(task_schedule_dto_to_app_dto)
        .transpose()?;

    let subtasks = payload.subtasks.map(|items| {
        items
            .into_iter()
            .map(|s| CreateSubtaskCommand {
                user_id: user.user_id,
                title: s.title,
                description: s.description,
            })
            .collect()
    });

    let priority = payload.priority.map(|p| match p {
        TaskPriorityDto::Low => DomainPriority::Low,
        TaskPriorityDto::Medium => DomainPriority::Medium,
        TaskPriorityDto::High => DomainPriority::High,
        TaskPriorityDto::Urgent => DomainPriority::Urgent,
    });

    let command = CreateTaskCommand {
        user_id: user.user_id,
        title: payload.title,
        description: payload.description,
        schedule,
        subtasks,
        category_id: payload.category_id,
        priority,
        reminders: payload.reminders.map(|items| {
            items
                .into_iter()
                .map(|r| AddReminderCommand {
                    date: DateTime::from_timestamp(r.date, 0).unwrap(),
                })
                .collect()
        }),
    };

    tracing::info!("Creating task with command: {:?}", command);

    let id = state.tasks.create_task_uc.execute(command).await?;
    tracing::info!("Task created successfully: {}", id);

    Ok(Json(CreateTaskResponseDto { id: id.to_string() }))
}
