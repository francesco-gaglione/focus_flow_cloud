use crate::adapters::http::model::session_model::{UserSession, SESSION_KEY};
use crate::adapters::http_error::HttpResult;
use crate::adapters::openapi::AUTH_TAG;
use crate::adapters::{http::app_state::AppState, http_error::HttpError};
use application::{app_error::AppError, use_cases::user::login_user::LoginCommand};
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginDto {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponseDto {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = AUTH_TAG,
    summary = "Login user",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Login successful", body = LoginResponseDto),
        (status = 401, description = "Unauthorized - Invalid credentials"),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login_api(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<LoginDto>,
) -> HttpResult<Json<LoginResponseDto>> {
    let cmd = LoginCommand {
        username: payload.username,
        password: payload.password,
    };

    let result = state.login_usecase.execute(cmd).await?;

    let session_data = UserSession {
        user_id: result.user_id,
    };

    session
        .insert(SESSION_KEY, session_data)
        .await
        .map_err(|e| HttpError::GenericError(format!("Session error: {}", e)))?;

    Ok(Json(LoginResponseDto {
        token: result.token,
    }))
}
