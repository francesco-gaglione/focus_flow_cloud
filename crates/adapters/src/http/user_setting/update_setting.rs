use crate::http::model::session_model::UserSession;
use axum::Extension;
use crate::http::app_state::AppState;
use crate::http_error::HttpResult;
use crate::openapi::SETTING_TAG;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserSettingDto {
    pub key: String,
    pub value: String,
}

#[utoipa::path(
    patch,
    path = "/api/setting",
    tag = SETTING_TAG,
    summary = "Update a user setting",
    request_body = UpdateUserSettingDto,
    responses(
        (status = 200, description = "User setting updated successfully"),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_setting_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Json(payload): Json<UpdateUserSettingDto>,
) -> HttpResult<()> {
    state
        .update_user_setting_usecase
        .execute(user.user_id, payload.key, payload.value)
        .await?;

    Ok(())
}
