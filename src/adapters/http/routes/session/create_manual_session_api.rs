use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::session_api::create_manual_session::{
    CreateManualSessionDto, CreateManualSessionResponseDto,
};
use crate::adapters::openapi::SESSION_TAG;
use crate::application::app_error::{AppError, AppResult};
use axum::Json;
use axum::extract::State;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/session/createManualSession",
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
    log::debug!("{:?}", payload);
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    //TODO

    Ok(Json(CreateManualSessionResponseDto { id: "".to_string() }))
}
