use dioxus::signals::{Signal, WritableExt};

use crate::clients::http_client::ApiClient;

pub enum UpdateBaseUrlError {
    ClientError(String),
}

pub type UpdateBaseUrlResult<T> = Result<T, UpdateBaseUrlError>;

pub fn update_base_url_uc(base_url: &str) -> UpdateBaseUrlResult<()> {
    let mut api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| "ApiClient signal not found".to_string())
        .map_err(|e| UpdateBaseUrlError::ClientError(e.to_string()))?;

    api_signal.write().set_base_url(base_url);

    Ok(())
}
