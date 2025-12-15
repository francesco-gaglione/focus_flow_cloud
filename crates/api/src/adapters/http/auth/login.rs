use crate::adapters::http::app_state::AppState;
use crate::adapters::http_error::HttpResult;
use crate::adapters::openapi::AUTH_TAG;
use application::use_cases::user::login_user::LoginCommand;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
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
    Json(payload): Json<LoginDto>,
) -> HttpResult<Json<LoginResponseDto>> {
    let cmd = LoginCommand {
        username: payload.username,
        password: payload.password,
    };

    let token = state.login_usecase.execute(cmd).await?;

    Ok(Json(LoginResponseDto { token }))
}
