use dioxus::signals::{ReadableExt, Signal, WritableExt};
use shared::auth::{LoginDto, LoginResponseDto, LogoutResponseDto, RefreshDto, RefreshResponseDto};

use crate::clients::http_client::{ApiClient, ApiError, ApiResult};

pub async fn login(username: &str, password: &str) -> ApiResult<LoginResponseDto> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| "ApiClient signal not found".to_string())
        .map_err(|e| ApiError::ClientError(e.to_string()))?;

    let request_dto = LoginDto {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = {
        let api = api_signal.read();
        api.post::<LoginDto, LoginResponseDto>("/api/auth/login", None, None, &request_dto)
            .await?
    };

    if let Some(mut api_signal) = dioxus::core::try_consume_context::<Signal<ApiClient>>() {
        api_signal
            .write()
            .set_auth_token(Some(response.token.clone()));
    }

    Ok(response)
}

pub async fn logout() -> ApiResult<LogoutResponseDto> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    api.post_raw::<(), LogoutResponseDto>("/api/auth/logout", None, None, &())
        .await
}

pub async fn refresh_api(refresh_token: &str) -> ApiResult<RefreshResponseDto> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;

    let dto = RefreshDto {
        refresh_token: refresh_token.to_string(),
    };

    let api = (*api_signal.read()).clone();
    // post_raw intentionally — must not trigger recursive refresh
    api.post_raw::<RefreshDto, RefreshResponseDto>("/api/auth/refresh", None, None, &dto)
        .await
}
