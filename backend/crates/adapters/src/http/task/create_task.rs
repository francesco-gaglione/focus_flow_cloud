use crate::http::app_state::AppState;
use crate::http::dto::common::task_dto::{
    task_schedule_app_dto_to_dto, task_schedule_dto_to_app_dto,
};
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::create_task::{
    CreateSubtaskCommand, CreateTaskCommand, CreateTaskError,
};
use axum::{extract::State, Extension, Json};
use chrono::DateTime;
use domain::entities::tasks::task_priority::TaskPriority as DomainPriority;
use shared::task::{CreateTaskDto, CreateTaskResponseDto, TaskPriority as SharedPriority};
use validator::Validate;

impl From<CreateTaskError> for HttpError {
    fn from(value: CreateTaskError) -> Self {
        match value {
            CreateTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
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
        .map(|s| task_schedule_dto_to_app_dto(s))
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
        SharedPriority::Low => DomainPriority::Low,
        SharedPriority::Medium => DomainPriority::Medium,
        SharedPriority::High => DomainPriority::High,
        SharedPriority::Urgent => DomainPriority::Urgent,
    });

    let command = CreateTaskCommand {
        user_id: user.user_id,
        title: payload.title,
        description: payload.description,
        schedule,
        subtasks,
        category_id: payload.category_id,
        priority,
    };

    tracing::info!("Creating task with command: {:?}", command);

    let id = state.create_task_uc.execute(command).await?;
    tracing::info!("Task created successfully: {}", id);

    Ok(Json(CreateTaskResponseDto { id: id.to_string() }))
}
