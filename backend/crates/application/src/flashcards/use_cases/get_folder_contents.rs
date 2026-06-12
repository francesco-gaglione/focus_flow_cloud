use crate::{
    flashcards::traits::flashcard_persistence::FlashcardPersistence,
    shared::traits::persistence_error::{PersistenceError, PersistenceResult},
};
use domain::flashcards::entities::flashcard::Flashcard;
use domain::shared::entities::folder_metadata::FolderMetadata;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum GetFolderContentsError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetFolderContentsResult<T> = Result<T, GetFolderContentsError>;

#[derive(Debug, Clone)]
pub struct FolderContents {
    pub folders: Vec<FolderMetadata>,
    pub flashcards: Vec<Flashcard>,
}

pub struct GetFolderContentsUseCase {
    flashcard_persistence: Arc<dyn FlashcardPersistence>,
}

impl GetFolderContentsUseCase {
    pub fn new(flashcard_persistence: Arc<dyn FlashcardPersistence>) -> Self {
        Self {
            flashcard_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute_for_root(
        &self,
        user_id: Uuid,
    ) -> GetFolderContentsResult<(Uuid, FolderContents)> {
        let root = self.get_or_create_root(user_id).await?;
        tracing::info!("Fetching root folder contents for user_id={}", user_id);
        let contents = self.execute(root.id()).await?;
        Ok((root.id(), contents))
    }

    async fn get_or_create_root(
        &self,
        user_id: Uuid,
    ) -> PersistenceResult<domain::shared::entities::folder_metadata::FolderMetadata> {
        match self
            .flashcard_persistence
            .find_root_folder_by_user(&user_id)
            .await
        {
            Ok(root) => Ok(root),
            Err(PersistenceError::NotFound(_)) => {
                tracing::info!("Root folder not found for user_id={}, creating...", user_id);
                self.flashcard_persistence
                    .create_root_folder_for_user(&user_id)
                    .await
            }
            Err(e) => Err(e),
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, folder_id: Uuid) -> GetFolderContentsResult<FolderContents> {
        tracing::info!("Fetching folder contents folder_id={}", folder_id);
        let folders = self
            .flashcard_persistence
            .find_subfolders_by_parent(&folder_id)
            .await?;
        let flashcards = self
            .flashcard_persistence
            .find_by_folder(&folder_id)
            .await?;

        Ok(FolderContents {
            folders,
            flashcards,
        })
    }
}
