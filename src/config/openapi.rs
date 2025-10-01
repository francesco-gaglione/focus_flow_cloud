use crate::api;
use crate::dto::category_api::create_category::{CreateCategoryDto, CreateCategoryResponseDto};
use crate::dto::category_api::get_categories::GetCategoriesResponseDto;
use crate::dto::task_api::create_task::{CreateTaskDto, CreateTaskResponseDto};
use crate::dto::task_api::delete_task::DeleteTasksDto;
use crate::dto::task_api::delete_task::DeleteTasksResponseDto;
use utoipa::OpenApi;

pub const CATEGORY_TAG: &str = "category";
pub const TASK_TAG: &str = "task";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = CATEGORY_TAG, description = "Category items management API"),
        (name = TASK_TAG, description = "Task items management API")
    ),
    paths(
        api::category::create_category_api::create_category_api,
        api::category::get_categories_and_tasks_api::get_categories_and_tasks_api,
        api::task::create_task_api::create_task_api,
        api::task::delete_tasks_api::delete_tasks_api,
    ),
    components(
        schemas(CreateCategoryDto, CreateCategoryResponseDto),
        schemas(CreateTaskDto, CreateTaskResponseDto),
        schemas(GetCategoriesResponseDto),
        schemas(DeleteTasksDto, DeleteTasksResponseDto),
    ),
    servers(
        (url = "/api", description = "API server")
    )
)]
pub struct ApiDoc;
