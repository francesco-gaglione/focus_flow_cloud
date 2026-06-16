use std::sync::Arc;

use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    flashcards::traits::flashcard_persistence::FlashcardPersistence,
    shared::traits::persistence_error::PersistenceError,
};
use domain::flashcards::entities::flashcard::Flashcard;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum GetFolderReviewQueueError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetFolderReviewQueueResult<T> = Result<T, GetFolderReviewQueueError>;

pub struct GetFolderReviewQueueUseCase {
    flashcard_persistence: Arc<dyn FlashcardPersistence>,
}

impl GetFolderReviewQueueUseCase {
    pub fn new(flashcard_persistence: Arc<dyn FlashcardPersistence>) -> Self {
        Self {
            flashcard_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, folder_id: Uuid) -> GetFolderReviewQueueResult<Vec<Flashcard>> {
        let cards = self
            .flashcard_persistence
            .find_due_by_folder(&folder_id)
            .await?;
        tracing::info!(
            "Found {} due flashcards for folder_id={}",
            cards.len(),
            folder_id
        );
        Ok(cards)
    }
}
