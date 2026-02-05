use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::USERS_TAG;
use application::use_cases::user::update_password::{
    UpdatePasswordError, UpdateUserPasswordCommand,
};
use axum::extract::{Extension, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

impl From<UpdatePasswordError> for HttpError {
    fn from(value: UpdatePasswordError) -> Self {
        match value {
            UpdatePasswordError::InvalidUserParam(e) => HttpError::BadRequest(e),
            UpdatePasswordError::InvalidPassword(password_policy_error) => {
                HttpError::BadRequest(password_policy_error.to_string())
            }
            UpdatePasswordError::UserNotFound(_) => {
                HttpError::NotFound("User not found".to_string())
            }
            UpdatePasswordError::PersistenceError(persistence_error) => {
                HttpError::GenericError(persistence_error.to_string())
            }
            UpdatePasswordError::HashingError(hashing_error) => {
                HttpError::GenericError(hashing_error.to_string())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdatePasswordDto {
    #[validate(length(min = 1, message = "Old password is required"))]
    pub old_password: String,
    #[validate(length(min = 1, message = "New password is required"))]
    pub new_password: String,
}

#[utoipa::path(
    put,
    path = "/api/users/password",
    tag = USERS_TAG,
    summary = "Update user password",
    request_body = UpdatePasswordDto,
    responses(
        (status = 200, description = "Password updated successfully"),
        (status = 400, description = "Bad request - validation error or invalid password"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_password_api(
    State(state): State<AppState>,
    Extension(user_session): Extension<UserSession>,
    Json(payload): Json<UpdatePasswordDto>,
) -> HttpResult<()> {
    let cmd = UpdateUserPasswordCommand {
        user_id: user_session.user_id,
        old_password: payload.old_password,
        new_password: payload.new_password,
    };

    state.update_password_usecase.execute(cmd).await?;

    Ok(())
}
