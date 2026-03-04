use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use crate::openapi::USERS_TAG;
use application::use_cases::user::get_user_info::UserInfoError;
use axum::extract::{Extension, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

impl From<UserInfoError> for HttpError {
    fn from(value: UserInfoError) -> Self {
        match value {
            UserInfoError::PersistenceError(e) => HttpError::GenericError(e.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserInfoResponseDto {
    pub id: uuid::Uuid,
    pub username: String,
    pub role: String,
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = USERS_TAG,
    summary = "Get current user info",
    responses(
        (status = 200, description = "User info retrieved", body = UserInfoResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_user_info_api(
    State(state): State<AppState>,
    Extension(user_session): Extension<UserSession>,
) -> HttpResult<Json<UserInfoResponseDto>> {
    let result = state
        .get_user_info_usecase
        .execute(user_session.user_id)
        .await?;

    Ok(Json(UserInfoResponseDto {
        id: result.id,
        username: result.username,
        role: result.role,
    }))
}
