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
pub enum GetFlashcardError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetFlashcardResult<T> = Result<T, GetFlashcardError>;

pub struct GetFlashcardUseCase {
    flashcard_persistence: Arc<dyn FlashcardPersistence>,
}

impl GetFlashcardUseCase {
    pub fn new(flashcard_persistence: Arc<dyn FlashcardPersistence>) -> Self {
        Self {
            flashcard_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, id: Uuid) -> GetFlashcardResult<Flashcard> {
        tracing::info!("Fetching flashcard id={}", id);
        let flashcard = self.flashcard_persistence.find_by_id(id).await?;
        Ok(flashcard)
    }
}

#[cfg(test)]
mod tests_get_flashcard_use_case {
    use std::sync::Arc;

    use domain::flashcards::entities::flashcard::Flashcard;
    use uuid::Uuid;

    use crate::{
        flashcards::{
            traits::flashcard_persistence::MockFlashcardPersistence,
            use_cases::get_flashcard::{GetFlashcardError, GetFlashcardUseCase},
        },
        shared::traits::persistence_error::PersistenceError,
    };

    #[tokio::test]
    async fn test_get_flashcard_success() {
        let mut mock = MockFlashcardPersistence::new();
        let user_id = Uuid::new_v4();

        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Flashcard::new("front", "back", user_id)));

        let uc = GetFlashcardUseCase::new(Arc::new(mock));
        let res = uc.execute(Uuid::new_v4()).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_flashcard_not_found() {
        let mut mock = MockFlashcardPersistence::new();

        mock.expect_find_by_id()
            .times(1)
            .returning(|_| Err(PersistenceError::NotFound("not found".to_string())));

        let uc = GetFlashcardUseCase::new(Arc::new(mock));
        let res = uc.execute(Uuid::new_v4()).await;

        assert!(matches!(
            res.err(),
            Some(GetFlashcardError::PersistenceError(_))
        ));
    }
}
