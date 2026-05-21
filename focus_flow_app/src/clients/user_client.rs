use dioxus::signals::{ReadableExt, Signal};
use serde::{Deserialize, Serialize};

use crate::clients::http_client::{ApiClient, ApiError, ApiResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUsernameDto {
    pub new_username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePasswordDto {
    pub old_password: String,
    pub new_password: String,
}

pub async fn update_username(new_username: &str) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let dto = UpdateUsernameDto {
        new_username: new_username.to_string(),
    };
    api.put::<_, ()>("/api/users/username", None, None, &dto)
        .await
}

pub async fn update_password(old_password: &str, new_password: &str) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let dto = UpdatePasswordDto {
        old_password: old_password.to_string(),
        new_password: new_password.to_string(),
    };
    api.put::<_, ()>("/api/users/password", None, None, &dto)
        .await
}
