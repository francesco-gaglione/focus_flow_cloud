use crate::adapters::http::dto::category_api::create_category::CreateCategoryDto;
use crate::adapters::http::dto::category_api::delete_categories::DeleteCategoriesDto;
use crate::adapters::http::dto::category_api::get_categories::GetCategoriesResponseDto;
use crate::adapters::http::dto::category_api::get_category::GetCategoryResponseDto;
use crate::adapters::http::dto::category_api::update_category::UpdateCategoryDto;
use crate::adapters::http::dto::category_api::update_category::UpdateCategoryResponseDto;
use crate::adapters::http::dto::session_api::create_manual_session::CreateManualSessionDto;
use crate::adapters::http::dto::session_api::create_manual_session::CreateManualSessionResponseDto;
use crate::adapters::http::dto::session_api::get_sessions::GetSessionFiltersDto;
use crate::adapters::http::dto::session_api::get_sessions::GetSessionFiltersResponseDto;
use crate::adapters::http::dto::stats_api::GetStatsByPeriodDto;
use crate::adapters::http::dto::stats_api::GetStatsByPeriodResponseDto;
use crate::adapters::http::dto::task_api::create_task::CreateTaskResponseDto;
use crate::adapters::http::dto::task_api::delete_task::DeleteTasksDto;
use crate::adapters::http::dto::task_api::orphan_tasks::OrphanTasksResponseDto;
use crate::adapters::http::dto::task_api::update_task::UpdateTaskDto;
use crate::adapters::http::dto::user_setting_api::get_user_settings::UserSettingsResponseDto;
use crate::adapters::http::dto::user_setting_api::update_setting::UpdateUserSettingDto;
use utoipa::OpenApi;

pub const CATEGORY_TAG: &str = "Category";
pub const TASK_TAG: &str = "Task";
pub const SESSION_TAG: &str = "Session";
pub const STATS_TAG: &str = "Statistics";
pub const SETTING_TAG: &str = "User Settings";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Focus flow app API",
        version = "0.1.0"
    ),
    tags(
        (name = CATEGORY_TAG, description = "Endpoints for managing categories and their tasks"),
        (name = TASK_TAG, description = "Endpoints for managing tasks"),
        (name = SESSION_TAG, description = "Endpoints for managing focus sessions"),
        (name = STATS_TAG, description = "Endpoints for retrieving statistics"),
        (name = SETTING_TAG, description = "Endpoints for managing user settings")
    ),
    paths(
        crate::adapters::http::routes::category::create_category_api::create_category_api,
        crate::adapters::http::routes::category::update_category_api::update_category_api,
        crate::adapters::http::routes::category::get_categories_and_tasks_api::get_categories_and_tasks_api,
        crate::adapters::http::routes::category::get_category::get_category,
        crate::adapters::http::routes::category::delete_categories_api::delete_categories_api,
        crate::adapters::http::routes::task::orphan_tasks_api::fetch_orphan_tasks_api,
        crate::adapters::http::routes::task::create_task_api::create_task_api,
        crate::adapters::http::routes::task::update_task_api::update_task_api,
        crate::adapters::http::routes::task::delete_tasks_api::delete_tasks_api,
        crate::adapters::http::routes::session::create_manual_session_api::create_manual_session_api,
        crate::adapters::http::routes::session::get_sessions::get_sessions,
        crate::adapters::http::routes::stats::calculate_stats_by_period::calculate_stats_by_period_api,
        crate::adapters::http::routes::user_setting::get_setting_api::get_settings_api,
        crate::adapters::http::routes::user_setting::update_setting_api::update_setting_api

    ),
    components(
        schemas(CreateCategoryDto, CreateCategoryDto),
        schemas(UpdateCategoryDto, UpdateCategoryResponseDto),
        schemas(UpdateTaskDto, CreateTaskResponseDto),
        schemas(DeleteCategoriesDto, GetCategoriesResponseDto),
        schemas(GetCategoryResponseDto),
        schemas(GetCategoriesResponseDto),
        schemas(OrphanTasksResponseDto),
        schemas(UpdateTaskDto, CreateTaskResponseDto),
        schemas(DeleteTasksDto, CreateTaskResponseDto),
        schemas(CreateManualSessionDto, CreateManualSessionResponseDto),
        schemas(GetSessionFiltersDto, GetSessionFiltersResponseDto),
        schemas(GetStatsByPeriodDto, GetStatsByPeriodResponseDto),
        schemas(UserSettingsResponseDto),
        schemas(UpdateUserSettingDto),

    ),
    servers(
        (url = "/api", description = "API server")
    )
)]
pub struct ApiDoc;
