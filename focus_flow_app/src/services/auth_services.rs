use dioxus::{
    prelude::info,
    signals::{ReadableExt, Signal, WritableExt},
};
use shared::auth::{LoginDto, LoginResponseDto};

use crate::{
    services::{
        api_client::{ApiClient, ApiError},
        storage::set_item,
    },
    state::auth_state::{self, AuthState},
};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Invalid credentials")]
    InvalidCredentials,
}

pub type AuthResult<T> = Result<T, AuthError>;

impl From<ApiError> for AuthError {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::Network(e) => AuthError::Network(e.to_string()),
            ApiError::InternalServerError(e) => AuthError::Generic(e.to_string()),
            ApiError::BadRequest(e) => AuthError::Generic(e.to_string()),
            ApiError::Unauthorized => AuthError::InvalidCredentials,
            ApiError::Deserialization(e) => AuthError::Generic(e.to_string()),
            ApiError::Unknown(e) => AuthError::Generic(e.to_string()),
        }
    }
}

pub async fn login(username: &str, password: &str) -> AuthResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| "ApiClient signal not found".to_string())
        .map_err(|e| AuthError::Generic(e.to_string()))?;

    let request_dto = LoginDto {
        username: username.to_string(),
        password: password.to_string(),
    };

    info!("Logging in with username: {}", username);
    let response = {
        let api = api_signal.read();
        api.post::<LoginDto, LoginResponseDto>("/api/auth/login", None, None, &request_dto)
            .await?
    };

    info!("Login successful, auth token received");

    let mut auth_state = dioxus::core::try_consume_context::<Signal<AuthState>>()
        .ok_or_else(|| "AuthState signal not found".to_string())
        .map_err(|e| AuthError::Generic(e.to_string()))?;

    let mut auth_state_wr = auth_state.write();

    auth_state_wr.set_auth_token(Some(response.token.clone()));
    auth_state_wr.set_refresh_token(Some(response.refresh_token.clone()));

    set_item("auth_token", response.token.as_str());
    set_item("refresh_token", response.refresh_token.as_str());

    if let Some(mut api_signal) = dioxus::core::try_consume_context::<Signal<ApiClient>>() {
        api_signal.write().set_auth_token(Some(response.token));
    }

    Ok(())
}
