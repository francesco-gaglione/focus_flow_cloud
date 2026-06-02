use crate::http::model::session_model::{UserSession, SESSION_KEY};
use crate::http_error::HttpResult;
use crate::openapi::AUTH_TAG;
use crate::{http::app_state::AppState, http_error::HttpError};
use application::use_cases::user::login_user::{LoginCommand, LoginError};
use axum::extract::State;
use axum::Json;
use secrecy::SecretBox;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct LoginDto {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDto {
    pub token: String,
    pub refresh_token: String,
}
use tower_sessions::Session;

impl From<LoginError> for HttpError {
    fn from(value: LoginError) -> Self {
        match value {
            LoginError::Validation(e) => HttpError::BadRequest(e.to_string()),
            LoginError::InvalidCredentials => HttpError::Unauthorized("".to_string()),
            LoginError::TokenError(_) => HttpError::Unauthorized("".to_string()),
        }
    }
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
        password: SecretBox::new(payload.password.into_boxed_str()),
    };

    let result = state.login_uc.execute(cmd).await?;

    let session_data = UserSession {
        user_id: result.user_id,
    };

    session
        .insert(SESSION_KEY, session_data)
        .await
        .map_err(|e| HttpError::GenericError(format!("Session error: {}", e)))?;

    Ok(Json(LoginResponseDto {
        token: result.token,
        refresh_token: result.refresh_token,
    }))
}
