use crate::adapters::http::app_state::AppState;
use crate::adapters::http_error::HttpResult;
use crate::adapters::openapi::USERS_TAG;
use application::use_cases::user::register_user::RegisterUserCommand;
use axum::extract::State;
use axum::Json;
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
    responses(
        (status = 200, description = "User created successfully"),
        (status = 400, description = "Bad request - validation error"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user_api(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> HttpResult<()> {
    let cmd = RegisterUserCommand {
        username: payload.username,
        password: payload.password,
    };

    state.register_user_usecase.execute(cmd).await?;

    Ok(())
}
