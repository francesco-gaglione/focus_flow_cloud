use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::session_api::create_manual_session::{
    CreateManualSessionDto, CreateManualSessionResponseDto,
};
use crate::adapters::mappers::focus_session_mapper::FocusSessionMapper;
use crate::adapters::openapi::SESSION_TAG;
use crate::application::app_error::{AppError, AppResult};
use axum::Json;
use axum::extract::State;
use tracing::debug;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/focusSession/createManualSession",
    tag = SESSION_TAG,
    request_body = CreateManualSessionDto,
    responses(
        (status = 200, description = "Session created successfully", body = CreateManualSessionResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 409, description = "Session already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_manual_session_api(
    State(state): State<AppState>,
    Json(payload): Json<CreateManualSessionDto>,
) -> AppResult<Json<CreateManualSessionResponseDto>> {
    debug!("{:?}", payload);
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let res = state
        .focus_session_use_cases
        .create_manual_session(FocusSessionMapper::manual_create_dto_to_command(&payload)?)
        .await?;

    Ok(Json(CreateManualSessionResponseDto {
        id: res.id.to_string(),
    }))
}
