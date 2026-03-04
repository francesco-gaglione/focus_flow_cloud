use crate::http::app_state::AppState;
use crate::http::dto::validators::validate_uuid::validate_uuid;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::TASK_TAG;
use application::use_cases::task::create_task::{CreateTaskCommand, CreateTaskError};

impl From<CreateTaskError> for HttpError {
    fn from(value: CreateTaskError) -> Self {
        match value {
            CreateTaskError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
            CreateTaskError::TaskError(e) => HttpError::BadRequest(e.to_string()),
        }
    }
}
use axum::{extract::State, Extension, Json};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskDto {
    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,

    pub scheduled_date: Option<i64>, //timestamp in seconds

    pub scheduled_end_date: Option<i64>, //timestamp in seconds
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
        (status = 409, description = "Task already exists"),
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

    let category_id = payload
        .category_id
        .as_ref()
        .map(|id| id.parse::<uuid::Uuid>())
        .transpose()
        .map_err(|_| HttpError::BadRequest("Invalid category id".to_string()))?;

    let scheduled_date = payload
        .scheduled_date
        .map(|s| {
            DateTime::from_timestamp(s, 0)
                .ok_or_else(|| HttpError::BadRequest("Invalid scheduled date".to_string()))
        })
        .transpose()?;
    let scheduled_end_date = payload
        .scheduled_end_date
        .map(|s| {
            DateTime::from_timestamp(s, 0)
                .ok_or_else(|| HttpError::BadRequest("Invalid scheduled end date".to_string()))
        })
        .transpose()?;

    let command = CreateTaskCommand {
        user_id: user.user_id,
        name: payload.name,
        description: payload.description,
        category_id,
        scheduled_date,
        scheduled_end_date,
    };

    let id = state.create_task_usecase.execute(command).await?;

    Ok(Json(CreateTaskResponseDto { id: id.to_string() }))
}
