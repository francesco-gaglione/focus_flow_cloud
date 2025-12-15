use crate::adapters::http::category::create_category::{
    CreateCategoryDto, CreateCategoryResponseDto,
};
use crate::adapters::http::category::delete_categories::DeleteCategoriesDto;
use crate::adapters::http::category::get_categories_and_tasks::GetCategoriesResponseDto;
use crate::adapters::http::category::get_category::GetCategoryResponseDto;
use crate::adapters::http::category::update_category::{
    UpdateCategoryDto, UpdateCategoryResponseDto,
};
use crate::adapters::http::session::create_manual_session::{
    CreateManualSessionDto, CreateManualSessionResponseDto,
};
use crate::adapters::http::session::get_sessions::{
    GetSessionFiltersDto, GetSessionFiltersResponseDto,
};
use crate::adapters::http::session::update_session::{
    UpdateFocusSessionDto, UpdateFocusSessionResponseDto,
};
use crate::adapters::http::stats::calculate_stats_by_period::{
    GetStatsByPeriodDto, GetStatsByPeriodResponseDto,
};
use crate::adapters::http::task::create_task::CreateTaskResponseDto;
use crate::adapters::http::task::delete_tasks::DeleteTasksDto;
use crate::adapters::http::task::get_tasks::TasksResponseDto;
use crate::adapters::http::task::orphan_tasks::OrphanTasksResponseDto;
use crate::adapters::http::task::update_task::UpdateTaskDto;
use crate::adapters::http::user_setting::get_user_settings::UserSettingsResponseDto;
use crate::adapters::http::user_setting::update_setting::UpdateUserSettingDto;
use crate::adapters::http::users::create_user::CreateUserDto;
use utoipa::OpenApi;

pub const CATEGORY_TAG: &str = "Category";
pub const TASK_TAG: &str = "Task";
pub const SESSION_TAG: &str = "Session";
pub const STATS_TAG: &str = "Statistics";
pub const SETTING_TAG: &str = "User Settings";
pub const USERS_TAG: &str = "Users";

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
        (name = SETTING_TAG, description = "Endpoints for managing user settings"),
        (name = USERS_TAG, description = "Endpoints for managing user")
    ),
    paths(
        crate::adapters::http::category::create_category::create_category_api,
        crate::adapters::http::category::update_category::update_category_api,
        crate::adapters::http::category::get_categories_and_tasks::get_categories_and_tasks_api,
        crate::adapters::http::category::get_category::get_category,
        crate::adapters::http::category::delete_categories::delete_categories_api,
        crate::adapters::http::task::orphan_tasks::fetch_orphan_tasks_api,
        crate::adapters::http::task::create_task::create_task_api,
        crate::adapters::http::users::create_user::create_user_api,
        crate::adapters::http::task::get_tasks::get_tasks_api,
        crate::adapters::http::task::update_task::update_task_api,
        crate::adapters::http::task::delete_tasks::delete_tasks_api,
        crate::adapters::http::session::create_manual_session::create_manual_session_api,
        crate::adapters::http::session::update_session::update_session_api,
        crate::adapters::http::session::get_sessions::get_sessions,
        crate::adapters::http::stats::calculate_stats_by_period::calculate_stats_by_period_api,
        crate::adapters::http::user_setting::get_user_settings::get_settings_api,
        crate::adapters::http::user_setting::update_setting::update_setting_api,
    ),
    components(
        schemas(CreateCategoryDto, CreateCategoryResponseDto),
        schemas(UpdateCategoryDto, UpdateCategoryResponseDto),
        schemas(UpdateTaskDto, CreateTaskResponseDto),
        schemas(DeleteCategoriesDto, GetCategoriesResponseDto),
        schemas(GetCategoryResponseDto),
        schemas(GetCategoriesResponseDto),
        schemas(OrphanTasksResponseDto),
        schemas(TasksResponseDto),
        schemas(UpdateTaskDto, CreateTaskResponseDto),
        schemas(DeleteTasksDto, CreateTaskResponseDto),
        schemas(CreateManualSessionDto, CreateManualSessionResponseDto),
        schemas(UpdateFocusSessionDto, UpdateFocusSessionResponseDto),
        schemas(GetSessionFiltersDto, GetSessionFiltersResponseDto),
        schemas(GetStatsByPeriodDto, GetStatsByPeriodResponseDto),
        schemas(UserSettingsResponseDto),
        schemas(UpdateUserSettingDto),
        schemas(CreateUserDto),
    ),
    servers(
        (url = "/", description = "API server")
    )
)]
pub struct ApiDoc;
