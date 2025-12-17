use crate::adapters::http::model::session_model::{UserSession, SESSION_KEY};
use crate::adapters::http_error::HttpResult;
use crate::adapters::openapi::USERS_TAG;
use crate::adapters::{http::app_state::AppState, http_error::HttpError};
use application::use_cases::user::register_user::RegisterUserCommand;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserDto {
    pub username: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/api/users",
    tag = USERS_TAG,
    summary = "Create a new user",
    request_body = CreateUserDto,
    responses(
        (status = 200, description = "User created successfully"),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user_api(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<CreateUserDto>,
) -> HttpResult<()> {
    let session_data: Option<UserSession> = session
        .get(SESSION_KEY)
        .await
        .map_err(|e| HttpError::GenericError(format!("Session error: {}", e)))?;

    let requester_id = match session_data {
        Some(s) => s.user_id,
        None => return Err(HttpError::Unauthorized("Missing session".to_string())),
    };

    let cmd = RegisterUserCommand {
        username: payload.username,
        password: payload.password,
        requester_user_id: requester_id,
    };

    state.register_user_usecase.execute(cmd).await?;

    Ok(())
}
