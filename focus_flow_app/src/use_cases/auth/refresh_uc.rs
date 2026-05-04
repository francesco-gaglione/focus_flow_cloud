use dioxus::signals::{ReadableExt, Signal, WritableExt};

use crate::{
    clients::{
        auth_client::refresh_api,
        http_client::ApiClient,
    },
    services::storage::{remove_item, set_item},
    state::auth_state::AuthState,
};

pub async fn refresh_uc() -> Result<(), ()> {
    let refresh_token = dioxus::core::try_consume_context::<Signal<AuthState>>()
        .and_then(|s| s.read().refresh_token().map(|t| t.to_string()))
        .ok_or(())?;

    match refresh_api(&refresh_token).await {
        Ok(response) => {
            if let Some(mut api_signal) = dioxus::core::try_consume_context::<Signal<ApiClient>>() {
                api_signal.write().set_auth_token(Some(response.token.clone()));
            }
            if let Some(mut auth_state) = dioxus::core::try_consume_context::<Signal<AuthState>>() {
                let mut wr = auth_state.write();
                wr.set_auth_token(Some(response.token.clone()));
                wr.set_refresh_token(Some(response.refresh_token.clone()));
            }
            set_item("auth_token", &response.token);
            set_item("refresh_token", &response.refresh_token);
            Ok(())
        }
        Err(_) => {
            if let Some(mut api_signal) = dioxus::core::try_consume_context::<Signal<ApiClient>>() {
                api_signal.write().set_auth_token(None);
            }
            if let Some(mut auth_state) = dioxus::core::try_consume_context::<Signal<AuthState>>() {
                let mut wr = auth_state.write();
                wr.delete_auth_token();
                wr.delete_refresh_token();
            }
            remove_item("auth_token");
            remove_item("refresh_token");
            Err(())
        }
    }
}
