use crate::clients::{category_http_client::create_category, http_client::ApiError};

#[derive(Debug, thiserror::Error)]
pub enum CreateCategoryError {
    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub async fn create_category_uc(name: &str, color: &str) -> Result<String, CreateCategoryError> {
    let id = create_category(name, color).await?;
    Ok(id)
}
