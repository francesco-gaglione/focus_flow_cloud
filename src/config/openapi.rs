use crate::api;
use crate::dto::create_category::{CreateCategoryDto, CreateCategoryResponseDto};
use utoipa::OpenApi;

pub const CATEGORY_TAG: &str = "category";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = CATEGORY_TAG, description = "Category items management API")
    ),
    paths(
        api::category::create_category_api::create_category_api
    ),
    components(
        schemas(CreateCategoryDto, CreateCategoryResponseDto)
    )
)]
pub struct ApiDoc;
