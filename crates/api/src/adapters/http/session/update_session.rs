use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::validators::validate_uuid::validate_uuid;
use crate::adapters::http_error::{HttpError, HttpResult};
use crate::adapters::mappers::focus_session_mapper::FocusSessionMapper;
use crate::adapters::openapi::SESSION_TAG;
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
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

    state
        .update_focus_session_usecase
        .execute(FocusSessionMapper::session_update_dto_to_command(
            path.id, &payload,
        )?)
        .await?;

    Ok(Json(UpdateFocusSessionResponseDto {}))
}
