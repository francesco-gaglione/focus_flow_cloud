use std::sync::Arc;

use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    flashcards::traits::flashcard_persistence::FlashcardPersistence,
    shared::traits::persistence_error::PersistenceError,
};
use domain::shared::entities::folder_metadata::FolderMetadata;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CreateFolderError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("folder name must be between 1 and 255 characters")]
    InvalidName,
}

pub type CreateFolderResult<T> = Result<T, CreateFolderError>;

#[derive(Debug)]
pub struct CreateFolderCommand {
    pub name: String,
    pub user_id: Uuid,
    pub parent_id: Uuid,
}

pub struct CreateFolderUseCase {
    flashcard_persistence: Arc<dyn FlashcardPersistence>,
}

impl CreateFolderUseCase {
    pub fn new(flashcard_persistence: Arc<dyn FlashcardPersistence>) -> Self {
        Self {
            flashcard_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        command: CreateFolderCommand,
    ) -> CreateFolderResult<FolderMetadata> {
        let trimmed = command.name.trim();
        if trimmed.is_empty() || trimmed.len() > 255 {
            return Err(CreateFolderError::InvalidName);
        }

        let folder = self
            .flashcard_persistence
            .create_folder(trimmed, &command.user_id, &command.parent_id)
            .await?;

        tracing::info!("Folder created id={} name={}", folder.id(), folder.name());
        Ok(folder)
    }
}
