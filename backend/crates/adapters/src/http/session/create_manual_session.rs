use crate::http::app_state::AppState;
use crate::http::dto::{
    common::session_type_enum::SessionTypeEnum, validators::validate_uuid::validate_uuid,
};
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::SESSION_TAG;
use application::use_cases::focus_session::create_manual_session::{
    CreateManualFocusSessionCommand, CreateManualSessionError,
};
use axum::{extract::State, Extension, Json};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

impl From<CreateManualSessionError> for HttpError {
    fn from(value: CreateManualSessionError) -> Self {
        match value {
            CreateManualSessionError::InvalidFocusSession(focus_session_error) => {
                HttpError::BadRequest(focus_session_error.to_string())
            }
            CreateManualSessionError::PersistenceError(persistence_error) => {
                HttpError::GenericError(persistence_error.to_string())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateManualSessionDto {
    #[validate(custom(function = "validate_uuid"))]
    pub task_id: Option<String>,

    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Option<String>,

    pub session_type: SessionTypeEnum,

    #[validate(range(min = 0, max = 5))]
    pub concentration_score: Option<i32>,

    //TODO validate
    pub started_at: i64, // timestamp in seconds

    pub ended_at: i64,

    #[validate(length(min = 1))]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateManualSessionResponseDto {
    pub id: String,
}

#[utoipa::path(
    post,
    path = "/api/focus-sessions/manual",
    tag = SESSION_TAG,
    summary = "Create a manual focus session",
    request_body = CreateManualSessionDto,
    responses(
        (status = 201, description = "Session created successfully", body = CreateManualSessionResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Session already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn create_manual_session_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Json(payload): Json<CreateManualSessionDto>,
) -> HttpResult<Json<CreateManualSessionResponseDto>> {
    debug!("{:?}", payload);
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let category_id = payload
        .category_id
        .map(|id| Uuid::parse_str(id.as_str()))
        .transpose()
        .map_err(|_| HttpError::BadRequest("Invalid category id".to_string()))?;
    let task_id = payload
        .task_id
        .map(|id| Uuid::parse_str(id.as_str()))
        .transpose()
        .map_err(|_| HttpError::BadRequest("Invalid task id".to_string()))?;

    let started_at = DateTime::from_timestamp(payload.started_at, 0)
        .ok_or_else(|| HttpError::BadRequest("Invalid timestamp".to_string()))?;

    let ended_at = DateTime::from_timestamp(payload.ended_at, 0)
        .ok_or_else(|| HttpError::BadRequest("Invalid timestamp".to_string()))?;

    let command = CreateManualFocusSessionCommand {
        user_id: user.user_id,
        category_id,
        task_id,
        session_type: payload.session_type.into(),
        concentration_score: payload.concentration_score,
        notes: payload.notes,
        started_at,
        ended_at,
    };

    let res = state.create_manual_session_usecase.execute(command).await?;

    Ok(Json(CreateManualSessionResponseDto {
        id: res.id().to_string(),
    }))
}
