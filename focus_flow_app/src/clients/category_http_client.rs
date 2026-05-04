use dioxus::signals::{ReadableExt, Signal};
use shared::category::GetAllCategoryResponseDto;

use crate::clients::http_client::{ApiClient, ApiError, ApiResult};

pub async fn get_all_categories() -> ApiResult<GetAllCategoryResponseDto> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    api.get("/api/category/categories", None, None).await
}
