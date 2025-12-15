use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::{
    common::session_type_enum::SessionTypeEnum, validators::validate_uuid::validate_uuid,
};
use crate::adapters::http_error::{HttpError, HttpResult};
use crate::adapters::mappers::focus_session_mapper::FocusSessionMapper;
use crate::adapters::openapi::SESSION_TAG;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;
use validator::Validate;

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

    //TODO should be validated?
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
        (status = 409, description = "Session already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_manual_session_api(
    State(state): State<AppState>,
    Json(payload): Json<CreateManualSessionDto>,
) -> HttpResult<Json<CreateManualSessionResponseDto>> {
    debug!("{:?}", payload);
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;

    let res = state
        .create_manual_session_usecase
        .execute(FocusSessionMapper::manual_create_dto_to_command(&payload)?)
        .await?;

    Ok(Json(CreateManualSessionResponseDto {
        id: res.id().to_string(),
    }))
}
