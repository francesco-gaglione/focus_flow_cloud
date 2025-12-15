use crate::adapters::http::app_state::AppState;
use crate::adapters::http::dto::common::user_setting_dto::UserSettingDto;
use crate::adapters::http_error::HttpResult;
use crate::adapters::openapi::SETTING_TAG;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSettingsResponseDto {
    pub settings: Vec<UserSettingDto>,
}

#[utoipa::path(
    get,
    path = "/api/user-settings",
    tag = SETTING_TAG,
    summary = "Fetch user settings",
    responses(
        (status = 200, description = "Settings updated successfully"),
        (status = 400, description = "Bad request - validation error"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_settings_api(
    State(state): State<AppState>,
) -> HttpResult<Json<UserSettingsResponseDto>> {
    let settings = state.get_user_settings_usecase.execute().await?;

    Ok(Json(UserSettingsResponseDto {
        settings: settings
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| UserSettingDto {
                key: s.key(),
                value: s.value().unwrap(),
            })
            .collect(),
    }))
}
