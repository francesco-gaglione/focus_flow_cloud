use crate::api;
use crate::dto::category_api::create_category::{CreateCategoryDto, CreateCategoryResponseDto};
use crate::dto::category_api::get_categories::GetCategoriesResponseDto;
use utoipa::OpenApi;

pub const CATEGORY_TAG: &str = "category";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = CATEGORY_TAG, description = "Category items management API")
    ),
    paths(
        api::category::create_category_api::create_category_api,
        api::category::get_categories::get_categories_and_tasks_api
    ),
    components(
        schemas(CreateCategoryDto, CreateCategoryResponseDto),
        schemas(GetCategoriesResponseDto)
    )
)]
pub struct ApiDoc;
