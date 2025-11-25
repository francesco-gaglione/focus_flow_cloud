use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::adapters::http::dto::common::user_setting_dto::UserSettingDto;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSettingsResponseDto {
    pub settings: Vec<UserSettingDto>,
}
