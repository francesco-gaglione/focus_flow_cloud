use crate::http::app_state::AppState;
use crate::http::dto::validators::validate_uuid::validate_uuid;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::SESSION_TAG;
use application::use_cases::focus_session::update_focus_session::{
    UpdateFocusSessionCommand, UpdateFocusSessionError,
};

impl From<UpdateFocusSessionError> for HttpError {
    fn from(value: UpdateFocusSessionError) -> Self {
        match value {
            UpdateFocusSessionError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
            UpdateFocusSessionError::FocusSessionError(e) => HttpError::BadRequest(e.to_string()),
        }
    }
}
use axum::extract::{Path, State};
use axum::Json;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateFocusSessionPathDto {
    #[validate(custom(function = "validate_uuid"))]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFocusSessionDto {
    #[validate(custom(function = "validate_uuid"))]
    pub category_id: Option<String>,

    #[validate(custom(function = "validate_uuid"))]
    pub task_id: Option<String>,

    #[validate(range(min = 0, max = 5))]
    pub concentration_score: Option<i32>,

    //TODO validate
    pub started_at: Option<i64>, // timestamp in seconds

    pub ended_at: Option<i64>,

    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateFocusSessionResponseDto {}

#[utoipa::path(
    put,
    path = "/api/focus-sessions/{id}",
    tag = SESSION_TAG,
    summary = "Update focus session",
    request_body = UpdateFocusSessionDto,
    responses(
        (status = 204, description = "Session updated successfully"),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Session already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_session_api(
    State(state): State<AppState>,
    Path(path): Path<UpdateFocusSessionPathDto>,
    Json(payload): Json<UpdateFocusSessionDto>,
) -> HttpResult<Json<UpdateFocusSessionResponseDto>> {
    debug!("{:?}", payload);

    path.validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let session_id = Uuid::parse_str(&path.id)
        .map_err(|_| HttpError::BadRequest("Invalid session id".to_string()))?;
    let category_id = payload
        .category_id
        .as_ref()
        .map(|id| Uuid::parse_str(id))
        .transpose()
        .map_err(|_| HttpError::BadRequest("Invalid category id".to_string()))?;
    let task_id = payload
        .task_id
        .as_ref()
        .map(|id| Uuid::parse_str(id))
        .transpose()
        .map_err(|_| HttpError::BadRequest("Invalid task id".to_string()))?;
    let started_at = payload
        .started_at
        .and_then(|s| DateTime::from_timestamp(s, 0))
        .ok_or_else(|| HttpError::BadRequest("Invalid started at timestamp".to_string()))?;
    let ended_at = payload
        .ended_at
        .and_then(|s| DateTime::from_timestamp(s, 0))
        .ok_or_else(|| HttpError::BadRequest("Invalid ended at timestamp".to_string()))?;

    let command = UpdateFocusSessionCommand {
        session_id,
        category_id,
        task_id,
        concentration_score: payload.concentration_score,
        notes: payload.notes,
        started_at: Some(started_at),
        ended_at: Some(ended_at),
    };

    state.update_focus_session_usecase.execute(command).await?;

    Ok(Json(UpdateFocusSessionResponseDto {}))
}
