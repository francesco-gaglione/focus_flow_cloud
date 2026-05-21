use dioxus::signals::{ReadableExt, Signal};
use serde::{Deserialize, Serialize};

use crate::clients::http_client::{ApiClient, ApiError, ApiResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoDto {
    pub id: String,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserDto {
    username: String,
    password: String,
}

pub async fn get_user_info() -> ApiResult<UserInfoDto> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    api.get("/api/users/me", None, None).await
}

pub async fn create_user(username: &str, password: &str) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let dto = CreateUserDto {
        username: username.to_string(),
        password: password.to_string(),
    };
    api.post::<_, ()>("/api/users", None, None, &dto).await
}
