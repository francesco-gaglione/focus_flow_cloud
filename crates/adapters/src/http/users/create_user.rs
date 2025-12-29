use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::HttpResult;
use crate::openapi::USERS_TAG;
use application::use_cases::user::register_user::RegisterUserCommand;
use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
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
    security(
        ("jwt" = [])
    ),
    responses(
        (status = 200, description = "User created successfully"),
        (status = 401, description = "Unauthorized - invalid credentials or not admin"),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user_api(
    State(state): State<AppState>,
    Extension(session_data): Extension<UserSession>,
    Json(payload): Json<CreateUserDto>,
) -> HttpResult<()> {
    let requester_id = session_data.user_id;

    let cmd = RegisterUserCommand {
        username: payload.username,
        password: payload.password,
        requester_user_id: requester_id,
    };

    state.register_user_usecase.execute(cmd).await?;

    Ok(())
}
