use std::sync::Arc;

use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    flashcards::traits::flashcard_persistence::FlashcardPersistence,
    shared::traits::persistence_error::PersistenceError,
};

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DeleteFolderError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type DeleteFolderResult<T> = Result<T, DeleteFolderError>;

pub struct DeleteFolderUseCase {
    flashcard_persistence: Arc<dyn FlashcardPersistence>,
}

impl DeleteFolderUseCase {
    pub fn new(flashcard_persistence: Arc<dyn FlashcardPersistence>) -> Self {
        Self {
            flashcard_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, id: Uuid) -> DeleteFolderResult<()> {
        self.flashcard_persistence.delete_folder(id).await?;
        tracing::info!("Folder deleted id={}", id);
        Ok(())
    }
}
