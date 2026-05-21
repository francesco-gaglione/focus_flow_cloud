use dioxus::signals::{Signal, WritableExt};

use crate::{
    clients::{auth_client::logout, http_client::ApiClient},
    services::storage::remove_item,
    state::auth_state::AuthState,
};

pub async fn logout_uc() {
    // Clear in-memory auth state
    if let Some(mut api_signal) = dioxus::core::try_consume_context::<Signal<ApiClient>>() {
        api_signal.write().set_auth_token(None);
    }
    if let Some(mut auth_state) = dioxus::core::try_consume_context::<Signal<AuthState>>() {
        let mut wr = auth_state.write();
        wr.delete_auth_token();
        wr.delete_refresh_token();
    }
    // Clear persisted tokens from storage
    remove_item("auth_token");
    remove_item("refresh_token");
    // Call API best-effort — ignore errors (session already cleared locally)
    let _ = logout().await;
}
