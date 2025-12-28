use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::HttpResult;
use crate::openapi::USERS_TAG;
use application::use_cases::user::update_user_username::UpdateUserUsernameCommand;
use axum::extract::{Extension, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUsernameDto {
    #[validate(length(min = 1, message = "New username is required"))]
    pub new_username: String,
}

#[utoipa::path(
    put,
    path = "/api/users/username",
    tag = USERS_TAG,
    summary = "Update user username",
    request_body = UpdateUsernameDto,
    responses(
        (status = 200, description = "Username updated successfully"),
        (status = 400, description = "Bad request - validation error or username already exists"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_username_api(
    State(state): State<AppState>,
    Extension(user_session): Extension<UserSession>,
    Json(payload): Json<UpdateUsernameDto>,
) -> HttpResult<()> {
    let cmd = UpdateUserUsernameCommand {
        user_id: user_session.user_id,
        new_username: payload.new_username,
    };

    state.update_user_username_usecase.execute(cmd).await?;

    Ok(())
}
