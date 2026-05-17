use std::future::Future;

use crate::{clients::http_client::{ApiError, ApiResult}, use_cases::auth::refresh_uc::refresh_uc};

pub async fn with_auth_retry<F, Fut, R>(f: F) -> ApiResult<R>
where
    F: Fn() -> Fut,
    Fut: Future<Output = ApiResult<R>>,
{
    match f().await {
        Err(ApiError::Unauthorized) => match refresh_uc().await {
            Ok(_) => f().await,
            Err(_) => Err(ApiError::Unauthorized),
        },
        other => other,
    }
}
