use crate::http_error::HttpResult;
use crate::openapi::AUTH_TAG;
use crate::{http::app_state::AppState, http_error::HttpError};
use application::use_cases::user::refresh_token::{RefreshTokenCommand, RefreshTokenError};
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

impl From<RefreshTokenError> for HttpError {
    fn from(value: RefreshTokenError) -> Self {
        match value {
            RefreshTokenError::TokenError(_) => HttpError::Unauthorized("".to_string()),
            RefreshTokenError::Validation(e) => HttpError::BadRequest(e),
            RefreshTokenError::InvalidCredentials => HttpError::Unauthorized("".to_string()),
            RefreshTokenError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RefreshDto {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshResponseDto {
    pub token: String,
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = AUTH_TAG,
    summary = "Refresh access token",
    request_body = RefreshDto,
    responses(
        (status = 200, description = "Refresh successful", body = RefreshResponseDto),
        (status = 401, description = "Unauthorized - Invalid refresh token"),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn refresh_api(
    State(state): State<AppState>,
    Json(payload): Json<RefreshDto>,
) -> HttpResult<Json<RefreshResponseDto>> {
    let cmd = RefreshTokenCommand {
        refresh_token: payload.refresh_token,
    };

    let result = state.refresh_token_uc.execute(cmd).await?;

    Ok(Json(RefreshResponseDto {
        token: result.token,
        refresh_token: result.refresh_token,
    }))
}
