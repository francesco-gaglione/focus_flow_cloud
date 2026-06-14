use application::flashcards::use_cases::create_folder::{CreateFolderCommand, CreateFolderError};
use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    http_error::{HttpError, HttpResult},
    openapi::FLASHCARD_TAG,
    shared::http::{app_state::AppState, model::session_model::UserSession},
};

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Folder name must be between 1 and 255 characters"
    ))]
    pub name: String,
    pub parent_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderResponseDto {
    pub id: Uuid,
    pub name: String,
}

impl From<CreateFolderError> for HttpError {
    fn from(value: CreateFolderError) -> Self {
        match value {
            CreateFolderError::PersistenceError(e) => {
                use application::shared::traits::persistence_error::PersistenceError;
                match e {
                    PersistenceError::NotFound(msg) => HttpError::NotFound(msg),
                    _ => HttpError::GenericError("Unexpected error".to_string()),
                }
            }
            CreateFolderError::InvalidName => {
                HttpError::BadRequest("Invalid folder name".to_string())
            }
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/flashcard/folder",
    tag = FLASHCARD_TAG,
    summary = "Create a new folder",
    request_body = CreateFolderDto,
    responses(
        (status = 201, description = "Folder created", body = CreateFolderResponseDto),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn create_folder_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Json(payload): Json<CreateFolderDto>,
) -> HttpResult<Json<CreateFolderResponseDto>> {
    payload
        .validate()
        .map_err(|e| HttpError::BadRequest(e.to_string()))?;
    tracing::info!("Creating folder name={}", payload.name);

    let command = CreateFolderCommand {
        name: payload.name,
        user_id: user.user_id,
        parent_id: payload.parent_id,
    };

    let folder = state.flashcards.create_folder_uc.execute(command).await?;

    Ok(Json(CreateFolderResponseDto {
        id: folder.id(),
        name: folder.name().to_string(),
    }))
}
