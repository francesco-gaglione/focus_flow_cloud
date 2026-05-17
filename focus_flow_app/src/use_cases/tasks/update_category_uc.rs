use crate::clients::{category_http_client::update_category, http_client::ApiError};

#[derive(Debug, thiserror::Error)]
pub enum UpdateCategoryError {
    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub async fn update_category_uc(
    id: &str,
    name: Option<&str>,
    color: Option<&str>,
) -> Result<(), UpdateCategoryError> {
    update_category(id, name, color).await?;
    Ok(())
}
