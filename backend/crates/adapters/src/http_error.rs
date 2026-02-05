use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde_json::json;
use thiserror::Error;

use crate::mappers::focus_session_mapper::FocusSessionMapperError;
use crate::mappers::task_mapper::TaskMapperError;

use domain::error::persistence_error::PersistenceError;

use application::use_cases::{
    category::{
        delete_categories_usecase::DeleteCategoriesError,
        delete_category_usecase::DeleteCategoryError,
        get_category_and_task_usecase::GetCategoryAndTasksError,
        get_category_usecase::GetCategoryError, update_category_usecase::UpdateCategoryError,
    },
    focus_session::{
        find_sessions_by_filters::FindSessionByFiltersError,
        update_focus_session::UpdateFocusSessionError,
    },
    stats::calculate_stats_by_period::CalculateStatsByPeriodError,
    task::{
        create_task::CreateTaskError, delete_tasks::DeleteTasksError, get_tasks::GetTaskError,
        orphan_tasks::OrphanTasksError, update_task::UpdateTaskError,
    },
    user::{
        get_user_info::UserInfoError, login_user::LoginError, refresh_token::RefreshTokenError,
        update_user_username::UpdateUserUsernameError,
    },
    user_settings::{get_settings::GetSettingsError, update_setting::UpdateSettingsError},
};

#[derive(Error, Debug, Clone)]
pub enum HttpError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Resource already exist: {0}")]
    ResourceAlreadyExist(String),

    #[error("Generic error: {0}")]
    GenericError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),

    #[error("Forbidden")]
    Forbidden,

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

pub type HttpResult<T> = Result<T, HttpError>;

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        tracing::error!("API Error: {:?}", self);

        let status = self.status_code();
        let error_code = self.error_code();
        let message = self.to_string();

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": message,
            }
        }));

        (status, body).into_response()
    }
}

impl HttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            HttpError::NotFound(_) => StatusCode::NOT_FOUND,
            HttpError::ResourceAlreadyExist(_) => StatusCode::CONFLICT,
            HttpError::BadRequest(_) => StatusCode::BAD_REQUEST,
            HttpError::Forbidden => StatusCode::FORBIDDEN,
            HttpError::GenericError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            HttpError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            HttpError::NotFound(_) => "NOT_FOUND",
            HttpError::ResourceAlreadyExist(_) => "CONFLICT",
            HttpError::BadRequest(_) => "BAD_REQUEST",
            HttpError::Forbidden => "FORBIDDEN",
            HttpError::GenericError(_) => "INTERNAL_ERROR",
            HttpError::Unauthorized(_) => "UNAUTHORIZED",
            HttpError::InvalidCredentials(_) => "UNAUTHORIZED",
        }
    }
}

impl From<FocusSessionMapperError> for HttpError {
    fn from(err: FocusSessionMapperError) -> Self {
        HttpError::BadRequest(err.to_string())
    }
}

impl From<TaskMapperError> for HttpError {
    fn from(err: TaskMapperError) -> Self {
        HttpError::BadRequest(err.to_string())
    }
}

// Helper to map PersistenceError to HttpError
fn map_persistence_error(err: PersistenceError) -> HttpError {
    match err {
        PersistenceError::NotFound(msg) => HttpError::NotFound(msg),
        PersistenceError::AlreadyExists => {
            HttpError::ResourceAlreadyExist("Resource already exists".to_string())
        }
        PersistenceError::Unexpected(msg) => HttpError::GenericError(msg),
    }
}

// CreateCategoryError: Implemented in controller

impl From<DeleteCategoriesError> for HttpError {
    fn from(err: DeleteCategoriesError) -> Self {
        match err {
            DeleteCategoriesError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<DeleteCategoryError> for HttpError {
    fn from(err: DeleteCategoryError) -> Self {
        match err {
            DeleteCategoryError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<GetCategoryAndTasksError> for HttpError {
    fn from(err: GetCategoryAndTasksError) -> Self {
        match err {
            GetCategoryAndTasksError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<GetCategoryError> for HttpError {
    fn from(err: GetCategoryError) -> Self {
        match err {
            GetCategoryError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<UpdateCategoryError> for HttpError {
    fn from(err: UpdateCategoryError) -> Self {
        match err {
            UpdateCategoryError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

// CreateManualSessionError: Implemented in controller

impl From<FindSessionByFiltersError> for HttpError {
    fn from(err: FindSessionByFiltersError) -> Self {
        match err {
            FindSessionByFiltersError::PersistenceError(e) => map_persistence_error(e),
            _ => HttpError::BadRequest(err.to_string()),
        }
    }
}

impl From<UpdateFocusSessionError> for HttpError {
    fn from(err: UpdateFocusSessionError) -> Self {
        match err {
            UpdateFocusSessionError::PersistenceError(e) => map_persistence_error(e),
            _ => HttpError::BadRequest(err.to_string()),
        }
    }
}

impl From<CalculateStatsByPeriodError> for HttpError {
    fn from(err: CalculateStatsByPeriodError) -> Self {
        match err {
            CalculateStatsByPeriodError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<CreateTaskError> for HttpError {
    fn from(err: CreateTaskError) -> Self {
        match err {
            CreateTaskError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<DeleteTasksError> for HttpError {
    fn from(err: DeleteTasksError) -> Self {
        match err {
            DeleteTasksError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<GetTaskError> for HttpError {
    fn from(err: GetTaskError) -> Self {
        match err {
            GetTaskError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<OrphanTasksError> for HttpError {
    fn from(err: OrphanTasksError) -> Self {
        match err {
            OrphanTasksError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<UpdateTaskError> for HttpError {
    fn from(err: UpdateTaskError) -> Self {
        match err {
            UpdateTaskError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<UserInfoError> for HttpError {
    fn from(err: UserInfoError) -> Self {
        match err {
            UserInfoError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<LoginError> for HttpError {
    fn from(err: LoginError) -> Self {
        HttpError::Unauthorized(err.to_string())
    }
}

impl From<RefreshTokenError> for HttpError {
    fn from(err: RefreshTokenError) -> Self {
        HttpError::Unauthorized(err.to_string())
    }
}

// RegisterUserError: Implemented in controller

// UpdatePasswordError: Implemented in controller

impl From<UpdateUserUsernameError> for HttpError {
    fn from(err: UpdateUserUsernameError) -> Self {
        match err {
            UpdateUserUsernameError::PersistenceError(e) => map_persistence_error(e),
            _ => HttpError::BadRequest(err.to_string()),
        }
    }
}

impl From<GetSettingsError> for HttpError {
    fn from(err: GetSettingsError) -> Self {
        match err {
            GetSettingsError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

impl From<UpdateSettingsError> for HttpError {
    fn from(err: UpdateSettingsError) -> Self {
        match err {
            UpdateSettingsError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}
