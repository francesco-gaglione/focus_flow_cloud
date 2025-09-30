use crate::services::service_error::ServiceError;
use axum::extract::rejection::JsonRejection;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Resource already exist: {0}")]
    ResourceAlreadyExist(String),

    #[error("Generic error: {0}")]
    GenericError(String),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    JsonValidationError(#[from] JsonRejection),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,
}

// Versione pulita con metodi helper
impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::ResourceAlreadyExist(_) => StatusCode::CONFLICT,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::JsonValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::GenericError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            ApiError::NotFound(_) => "NOT_FOUND",
            ApiError::ResourceAlreadyExist(_) => "CONFLICT",
            ApiError::ValidationError(_) => "BAD_REQUEST",
            ApiError::JsonValidationError(_) => "BAD_REQUEST",
            ApiError::Unauthorized => "UNAUTHORIZED",
            ApiError::Forbidden => "FORBIDDEN",
            ApiError::GenericError(_) => "INTERNAL_ERROR",
        }
    }
}

impl IntoResponse for ApiError {
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

// IL TUO FROM RIMANE IDENTICO!
impl From<ServiceError> for ApiError {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::CategoryAlreadyExists => {
                ApiError::ResourceAlreadyExist(String::from("Category already exists"))
            }
            ServiceError::CategoryNotFound => {
                ApiError::NotFound(String::from("Category not found"))
            }
            ServiceError::RepositoryError(err) => {
                ApiError::GenericError(String::from("Generic error happened, try again later"))
            }
        }
    }
}
