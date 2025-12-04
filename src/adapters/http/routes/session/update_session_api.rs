use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::session_api::update_session::{
    UpdateFocusSessionDto, UpdateFocusSessionPathDto, UpdateFocusSessionResponseDto,
};
use crate::adapters::http_error::{HttpError, HttpResult};
use crate::adapters::mappers::focus_session_mapper::FocusSessionMapper;
use crate::adapters::openapi::SESSION_TAG;
use axum::extract::{Path, State};
use axum::Json;
use tracing::debug;
use validator::Validate;

#[utoipa::path(
    put,
    path = "/api/focus-sessions/{id}",
    tag = SESSION_TAG,
    summary = "Update focus session",
    request_body = UpdateFocusSessionDto,
    responses(
        (status = 204, description = "Session updated successfully"),
        (status = 400, description = "Bad request - validation error"),
        (status = 409, description = "Session already exists"),
        (status = 500, description = "Internal server error")
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

    let _ = state
        .update_focus_session_usecase
        .execute(FocusSessionMapper::session_update_dto_to_command(
            path.id, &payload,
        )?)
        .await?;

    Ok(Json(UpdateFocusSessionResponseDto {}))
}
