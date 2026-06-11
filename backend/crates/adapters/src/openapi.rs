use crate::user::http::auth::login::{LoginDto, LoginResponseDto};
use crate::user::http::auth::logout::LogoutResponseDto;
use crate::user::http::auth::oauth_token::{OAuthTokenForm, OAuthTokenResponse};
use crate::user::http::auth::refresh::{RefreshDto, RefreshResponseDto};
use crate::tasks::http::category::create_category::{CreateCategoryDto, CreateCategoryResponseDto};
use crate::tasks::http::category::delete_categories::{DeleteCategoriesDto, DeleteCategoriesResponseDto};
use crate::tasks::http::category::get_all_categories::GetAllCategoryResponseDto;
use crate::tasks::http::category::update_category::{UpdateCategoryDto, UpdateCategoryResponseDto};
use crate::tasks::http::session::create_manual_session::{
    CreateManualSessionDto, CreateManualSessionResponseDto,
};
use crate::tasks::http::session::get_sessions::{GetSessionFiltersDto, GetSessionFiltersResponseDto};
use crate::tasks::http::session::update_session::{UpdateFocusSessionDto, UpdateFocusSessionResponseDto};
use crate::tasks::http::stats::get_stats::GetStatsResponseDto;
use crate::tasks::http::task::create_subtask::{CreateSubtaskDto, CreateSubtaskResponseDto};
use crate::tasks::http::task::create_task::{CreateTaskDto, CreateTaskResponseDto};
use crate::tasks::http::task::delete_tasks::{DeleteTaskResponseDto, DeleteTasksDto};
use crate::tasks::http::task::get_tasks::TasksResponseDto;
use crate::tasks::http::task::update_subtask::{UpdateSubTaskDto, UpdateSubTaskResponseDto};
use crate::tasks::http::task::update_task::{UpdateTaskDto, UpdateTaskResponseDto};
use crate::user::http::user_setting::get_user_settings::UserSettingsResponseDto;
use crate::user::http::user_setting::update_setting::UpdateUserSettingDto;
use crate::user::http::users::create_user::CreateUserDto;
use crate::user::http::users::get_info::UserInfoResponseDto;
use crate::user::http::users::update_password::UpdatePasswordDto;
use crate::user::http::users::update_username::UpdateUsernameDto;
use utoipa::openapi::security::{
    Flow, HttpAuthScheme, HttpBuilder, OAuth2, Password, Scopes, SecurityScheme,
};
use utoipa::{Modify, OpenApi};

pub const AUTH_TAG: &str = "Auth";
pub const CATEGORY_TAG: &str = "Category";
pub const TASK_TAG: &str = "Task";
pub const SESSION_TAG: &str = "Session";
pub const STATS_TAG: &str = "Statistics";
pub const SETTING_TAG: &str = "User Settings";
pub const USERS_TAG: &str = "Users";

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
            components.add_security_scheme(
                "oauth2",
                SecurityScheme::OAuth2(OAuth2::new([Flow::Password(Password::new(
                    "/api/auth/token",
                    Scopes::new(),
                ))])),
            );
        }
    }
}

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
        crate::tasks::http::category::create_category::create_category_api,
        crate::tasks::http::category::update_category::update_category_api,
        crate::tasks::http::category::delete_categories::delete_categories_api,
        crate::tasks::http::category::get_all_categories::get_all_categories_api,
        crate::user::http::users::create_user::create_user_api,
        crate::user::http::users::update_password::update_password_api,
        crate::user::http::users::update_username::update_username_api,
        crate::user::http::users::get_info::get_user_info_api,
        crate::user::http::auth::login::login_api,
        crate::user::http::auth::oauth_token::oauth_token_api,
        crate::user::http::auth::refresh::refresh_api,
        crate::user::http::auth::logout::logout_api,
        crate::tasks::http::task::get_tasks::get_tasks_api,
        crate::tasks::http::task::update_task::update_task_api,
        crate::tasks::http::task::update_subtask::update_subtask_api,
        crate::tasks::http::task::create_subtask::create_subtask_api,
        crate::tasks::http::task::delete_tasks::delete_tasks_api,
        crate::tasks::http::task::create_task::create_task_api,
        crate::tasks::http::session::create_manual_session::create_manual_session_api,
        crate::tasks::http::session::update_session::update_session_api,
        crate::tasks::http::session::get_sessions::get_sessions,
        crate::tasks::http::stats::get_stats::get_stats_api,
        crate::user::http::user_setting::get_user_settings::get_settings_api,
        crate::user::http::user_setting::update_setting::update_setting_api,
    ),
    components(
        schemas(CreateCategoryDto, CreateCategoryResponseDto),
        schemas(UpdateCategoryDto, UpdateCategoryResponseDto),
        schemas(UpdateTaskDto, UpdateTaskResponseDto, CreateTaskResponseDto),
        schemas(DeleteCategoriesDto, DeleteCategoriesResponseDto),
        schemas(GetAllCategoryResponseDto),
        schemas(TasksResponseDto),
        schemas(UpdateSubTaskDto, UpdateSubTaskResponseDto),
        schemas(CreateSubtaskDto, CreateSubtaskResponseDto),
        schemas(CreateTaskDto),
        schemas(DeleteTasksDto, DeleteTaskResponseDto),
        schemas(CreateManualSessionDto, CreateManualSessionResponseDto),
        schemas(UpdateFocusSessionDto, UpdateFocusSessionResponseDto),
        schemas(GetSessionFiltersDto, GetSessionFiltersResponseDto),
        schemas(GetStatsResponseDto),
        schemas(UserSettingsResponseDto),
        schemas(UpdateUserSettingDto),
        schemas(CreateUserDto),
        schemas(UpdatePasswordDto),
        schemas(UpdateUsernameDto),
        schemas(UserInfoResponseDto),
        schemas(LoginDto, LoginResponseDto),
        schemas(OAuthTokenForm, OAuthTokenResponse),
        schemas(RefreshDto, RefreshResponseDto),
        schemas(LogoutResponseDto),
    ),
    servers(
        (url = "/", description = "API server")
    )
)]
pub struct ApiDoc;
