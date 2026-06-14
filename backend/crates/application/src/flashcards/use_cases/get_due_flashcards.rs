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
pub enum GetDueFlashcardsError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetDueFlashcardsResult<T> = Result<T, GetDueFlashcardsError>;

pub struct GetDueFlashcardsUseCase {
    flashcard_persistence: Arc<dyn FlashcardPersistence>,
}

impl GetDueFlashcardsUseCase {
    pub fn new(flashcard_persistence: Arc<dyn FlashcardPersistence>) -> Self {
        Self {
            flashcard_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, user_id: Uuid) -> GetDueFlashcardsResult<Vec<Flashcard>> {
        let cards = self
            .flashcard_persistence
            .find_due_by_user(&user_id)
            .await?;
        tracing::info!(
            "Found {} due flashcards for user_id={}",
            cards.len(),
            user_id
        );
        Ok(cards)
    }
}
