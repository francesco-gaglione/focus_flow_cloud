use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde_json::json;
use thiserror::Error;

use application::app_error::AppError;

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
        }
    }
}

impl From<AppError> for HttpError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::NotFound(msg) => HttpError::NotFound(msg),
            AppError::ResourceAlreadyExist(msg) => HttpError::ResourceAlreadyExist(msg),
            AppError::GenericError(msg) => HttpError::GenericError(msg),
            AppError::BadRequest(msg) => HttpError::BadRequest(msg),
            AppError::Forbidden => HttpError::Forbidden,
            AppError::Database(msg) => HttpError::GenericError(msg),
            AppError::InvalidFocusSessionDuration => {
                HttpError::BadRequest("Invalid date range".to_string())
            }
            AppError::Persistence(msg) => HttpError::GenericError(msg),
            AppError::InvalidColor(msg) => HttpError::GenericError(msg),
            AppError::InvalidFocusSessionParam(msg) => HttpError::BadRequest(msg),
            AppError::InvalidDateRange(msg) => HttpError::BadRequest(msg),
            AppError::InvalidId(msg) => HttpError::BadRequest(msg),
            AppError::InvalidStatsParam(msg) => HttpError::BadRequest(msg),
            AppError::InvalidUserParam(msg) => HttpError::BadRequest(msg),
            AppError::InvalidPasswordPolicy(msg) => HttpError::BadRequest(msg),
            AppError::PasswordHashingError(msg) => HttpError::GenericError(msg),
            AppError::UserNotFound(msg) => HttpError::NotFound(msg),
            AppError::TokenError(msg) => HttpError::GenericError(msg),
            AppError::Validation(msg) => HttpError::BadRequest(msg),
            AppError::Unauthorized(msg) => HttpError::Unauthorized(msg),
        }
    }
}
