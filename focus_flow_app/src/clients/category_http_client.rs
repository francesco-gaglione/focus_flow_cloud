use dioxus::signals::{ReadableExt, Signal};
use shared::category::{
    CreateCategoryDto, CreateCategoryResponseDto, GetAllCategoryResponseDto, UpdateCategoryDto,
};

use crate::clients::http_client::{ApiClient, ApiError, ApiResult};

pub async fn get_all_categories() -> ApiResult<GetAllCategoryResponseDto> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    api.get("/api/category/categories", None, None).await
}

pub async fn create_category(category_name: &str, color: &str) -> ApiResult<String> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api_client = (*api_signal.read()).clone();

    let dto = CreateCategoryDto {
        name: category_name.to_string(),
        color: color.to_string(),
    };

    let response = api_client
        .post::<CreateCategoryDto, CreateCategoryResponseDto>("/api/category", None, None, &dto)
        .await?;

    Ok(response.category_id)
}

pub async fn delete_category(category_id: &str) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api_client = (*api_signal.read()).clone();

    api_client
        .delete(
            "/api/category/{category_id}",
            Some(&[("category_id", category_id)]),
            None,
        )
        .await
}

pub async fn update_category(
    category_id: &str,
    category_name: Option<&str>,
    color: Option<&str>,
) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api_client = (*api_signal.read()).clone();

    let dto = UpdateCategoryDto {
        name: category_name.map(|n| n.to_string()),
        color: color.map(|c| c.to_string()),
    };

    api_client
        .patch(
            "/api/category/{category_id}",
            Some(&[("category_id", category_id)]),
            None,
            &dto,
        )
        .await
}
