use crate::http::auth::login::{LoginDto, LoginResponseDto};
use crate::http::auth::logout::LogoutResponseDto;
use crate::http::auth::refresh::{RefreshDto, RefreshResponseDto};
use crate::http::category::create_category::{CreateCategoryDto, CreateCategoryResponseDto};
use crate::http::category::delete_categories::DeleteCategoriesDto;
use crate::http::category::get_categories_and_tasks::GetCategoriesResponseDto;
use crate::http::category::get_category::GetCategoryResponseDto;
use crate::http::category::update_category::{UpdateCategoryDto, UpdateCategoryResponseDto};
use crate::http::session::create_manual_session::{
    CreateManualSessionDto, CreateManualSessionResponseDto,
};
use crate::http::session::get_sessions::{GetSessionFiltersDto, GetSessionFiltersResponseDto};
use crate::http::session::update_session::{UpdateFocusSessionDto, UpdateFocusSessionResponseDto};
use crate::http::stats::calculate_stats_by_period::{
    GetStatsByPeriodDto, GetStatsByPeriodResponseDto,
};
use crate::http::task::create_task::CreateTaskResponseDto;
use crate::http::task::delete_tasks::DeleteTasksDto;
use crate::http::task::get_tasks::TasksResponseDto;
use crate::http::task::orphan_tasks::OrphanTasksResponseDto;
use crate::http::task::update_task::UpdateTaskDto;
use crate::http::user_setting::get_user_settings::UserSettingsResponseDto;
use crate::http::user_setting::update_setting::UpdateUserSettingDto;
use crate::http::users::create_user::CreateUserDto;
use crate::http::users::get_info::UserInfoResponseDto;
use crate::http::users::update_password::UpdatePasswordDto;
use crate::http::users::update_username::UpdateUsernameDto;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

pub const AUTH_TAG: &str = "Auth";

// ... existing tags ...

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

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
    modifiers(&SecurityAddon),
    tags(
        (name = CATEGORY_TAG, description = "Endpoints for managing categories and their tasks"),
        (name = TASK_TAG, description = "Endpoints for managing tasks"),
        (name = SESSION_TAG, description = "Endpoints for managing focus sessions"),
        (name = STATS_TAG, description = "Endpoints for retrieving statistics"),
        (name = SETTING_TAG, description = "Endpoints for managing user settings"),
        (name = USERS_TAG, description = "Endpoints for managing user"),
        (name = AUTH_TAG, description = "Endpoints for authentication")
    ),
    paths(
        crate::http::category::create_category::create_category_api,
        crate::http::category::update_category::update_category_api,
        crate::http::category::get_categories_and_tasks::get_categories_and_tasks_api,
        crate::http::category::get_category::get_category,
        crate::http::category::delete_categories::delete_categories_api,
        crate::http::task::orphan_tasks::fetch_orphan_tasks_api,
        crate::http::task::create_task::create_task_api,
        crate::http::users::create_user::create_user_api,
        crate::http::users::update_password::update_password_api,
        crate::http::users::update_username::update_username_api,
        crate::http::users::get_info::get_user_info_api,
        crate::http::auth::login::login_api,
        crate::http::auth::refresh::refresh_api,
        crate::http::auth::logout::logout_api,
        crate::http::task::get_tasks::get_tasks_api,
        crate::http::task::update_task::update_task_api,
        crate::http::task::delete_tasks::delete_tasks_api,
        crate::http::session::create_manual_session::create_manual_session_api,
        crate::http::session::update_session::update_session_api,
        crate::http::session::get_sessions::get_sessions,
        crate::http::stats::calculate_stats_by_period::calculate_stats_by_period_api,
        crate::http::user_setting::get_user_settings::get_settings_api,
        crate::http::user_setting::update_setting::update_setting_api,
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
        schemas(UpdatePasswordDto),
        schemas(UpdateUsernameDto),
        schemas(UserInfoResponseDto),
        schemas(LoginDto, LoginResponseDto),
        schemas(RefreshDto, RefreshResponseDto),
        schemas(LogoutResponseDto),
    ),
    servers(
        (url = "/", description = "API server")
    )
)]
pub struct ApiDoc;
