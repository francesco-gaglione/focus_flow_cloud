use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::create_task::{CreateTaskCommand, CreateTaskError};
use axum::{extract::State, Extension, Json};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

impl From<CreateTaskError> for HttpError {
    fn from(value: CreateTaskError) -> Self {
        match value {
            CreateTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
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

    pub due_date: Option<i64>, // timestamp in seconds
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTaskResponseDto {
    pub id: String,
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

    let due_date = payload
        .due_date
        .map(|s| {
            DateTime::from_timestamp(s, 0).ok_or_else(|| {
                tracing::error!("Invalid due date: {}", s);
                HttpError::BadRequest("Invalid due date".to_string())
            })
        })
        .transpose()?;

    let command = CreateTaskCommand {
        user_id: user.user_id,
        title: payload.title,
        description: payload.description,
        due_date,
    };

    tracing::info!("Creating task with command: {:?}", command);

    let id = state.create_task_uc.execute(command).await?;
    tracing::info!("Task created successfully: {}", id);

    Ok(Json(CreateTaskResponseDto { id: id.to_string() }))
}
