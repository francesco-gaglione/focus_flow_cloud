use crate::clients::{category_http_client::delete_category, http_client::ApiError};

#[derive(Debug, thiserror::Error)]
pub enum DeleteCategoryError {
    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

pub async fn delete_category_uc(id: &str) -> Result<(), DeleteCategoryError> {
    delete_category(id).await?;
    Ok(())
}
