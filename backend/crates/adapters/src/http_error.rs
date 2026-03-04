use application::persistence_traits::persistence_error::PersistenceError;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde_json::json;
use thiserror::Error;

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

// Helper to map PersistenceError to HttpError
pub fn map_persistence_error(err: PersistenceError) -> HttpError {
    match err {
        PersistenceError::NotFound(msg) => HttpError::NotFound(msg),
        PersistenceError::AlreadyExists => {
            HttpError::ResourceAlreadyExist("Resource already exists".to_string())
        }
        PersistenceError::Unexpected(msg) => HttpError::GenericError(msg),
    }
}
