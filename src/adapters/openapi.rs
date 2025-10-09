use crate::adapters::http::dto::category_api::create_category::CreateCategoryDto;
use crate::adapters::http::dto::category_api::delete_categories::DeleteCategoriesDto;
use crate::adapters::http::dto::category_api::get_categories::GetCategoriesResponseDto;
use crate::adapters::http::dto::session_api::create_manual_session::CreateManualSessionDto;
use crate::adapters::http::dto::session_api::create_manual_session::CreateManualSessionResponseDto;
use crate::adapters::http::dto::session_api::get_sessions::GetSessionByDateRangeResponseDto;
use crate::adapters::http::dto::session_api::get_sessions::GetSessionFiltersDto;
use crate::adapters::http::dto::task_api::create_task::CreateTaskResponseDto;
use crate::adapters::http::dto::task_api::delete_task::DeleteTasksDto;
use crate::adapters::http::dto::task_api::update_task::UpdateTaskDto;
use utoipa::OpenApi;

pub const CATEGORY_TAG: &str = "Category";
pub const TASK_TAG: &str = "Task";
pub const SESSION_TAG: &str = "Session";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Focus flow app API",
        version = "0.1.0"
    ),
    tags(
        (name = CATEGORY_TAG, description = "Category items management API"),
        (name = TASK_TAG, description = "Task items management API")
    ),
    paths(
        crate::adapters::http::routes::category::create_category_api::create_category_api,
        crate::adapters::http::routes::category::get_categories_and_tasks_api::get_categories_and_tasks_api,
        crate::adapters::http::routes::category::delete_categories_api::delete_categories_api,
        crate::adapters::http::routes::task::create_task_api::create_task_api,
        crate::adapters::http::routes::task::update_task_api::update_task_api,
        crate::adapters::http::routes::task::delete_tasks_api::delete_tasks_api,
        crate::adapters::http::routes::session::create_manual_session_api::create_manual_session_api,
        crate::adapters::http::routes::session::get_sessions::get_sessions,
    ),
    components(
        schemas(CreateCategoryDto, CreateCategoryDto),
        schemas(UpdateTaskDto, CreateTaskResponseDto),
        schemas(DeleteCategoriesDto, GetCategoriesResponseDto),
        schemas(GetCategoriesResponseDto),
        schemas(UpdateTaskDto, CreateTaskResponseDto),
        schemas(DeleteTasksDto, CreateTaskResponseDto),
        schemas(CreateManualSessionDto, CreateManualSessionResponseDto),
        schemas(GetSessionFiltersDto, GetSessionByDateRangeResponseDto),
    ),
    servers(
        (url = "/api", description = "API server")
    )
)]
pub struct ApiDoc;
