use dioxus::{
    logger::tracing::info,
    signals::{Signal, WritableExt},
};

use crate::{
    clients::auth_client::login, services::storage::set_item, state::auth_state::AuthState,
};

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("Generic error: {0}")]
    Generic(String),
}

pub type LoginResult<T> = Result<T, LoginError>;

pub async fn login_uc(username: &str, password: &str) -> LoginResult<()> {
    info!("Logging in with username: {}", username);
    let response = login(username, password)
        .await
        .map_err(|e| LoginError::Generic(e.to_string()))?;

    info!("Login successful, auth token received");

    let mut auth_state = dioxus::core::try_consume_context::<Signal<AuthState>>()
        .ok_or_else(|| "AuthState signal not found".to_string())
        .map_err(|e| LoginError::Generic(e.to_string()))?;

    let mut auth_state_wr = auth_state.write();

    auth_state_wr.set_auth_token(Some(response.token.clone()));
    auth_state_wr.set_refresh_token(Some(response.refresh_token.clone()));

    set_item("auth_token", response.token.as_str());
    set_item("refresh_token", response.refresh_token.as_str());

    Ok(())
}
