use std::sync::Arc;

use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    flashcards::traits::flashcard_persistence::FlashcardPersistence,
    shared::traits::persistence_error::PersistenceError,
};

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DeleteFlashcardError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type DeleteFlashcardResult<T> = Result<T, DeleteFlashcardError>;

pub struct DeleteFlashcardUseCase {
    flashcard_persistence: Arc<dyn FlashcardPersistence>,
}

impl DeleteFlashcardUseCase {
    pub fn new(flashcard_persistence: Arc<dyn FlashcardPersistence>) -> Self {
        Self {
            flashcard_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, id: Uuid) -> DeleteFlashcardResult<()> {
        self.flashcard_persistence.delete(id).await?;
        tracing::info!("Flashcard deleted id={}", id);
        Ok(())
    }
}

#[cfg(test)]
mod tests_delete_flashcard_use_case {
    use std::sync::Arc;

    use uuid::Uuid;

    use crate::{
        flashcards::{
            traits::flashcard_persistence::MockFlashcardPersistence,
            use_cases::delete_flashcard::{DeleteFlashcardError, DeleteFlashcardUseCase},
        },
        shared::traits::persistence_error::PersistenceError,
    };

    #[tokio::test]
    async fn test_delete_flashcard_success() {
        let mut mock = MockFlashcardPersistence::new();

        mock.expect_delete()
            .times(1)
            .returning(|_| Ok(()));

        let uc = DeleteFlashcardUseCase::new(Arc::new(mock));
        let res = uc.execute(Uuid::new_v4()).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_delete_flashcard_not_found() {
        let mut mock = MockFlashcardPersistence::new();

        mock.expect_delete()
            .times(1)
            .returning(|_| Err(PersistenceError::NotFound("not found".to_string())));

        let uc = DeleteFlashcardUseCase::new(Arc::new(mock));
        let res = uc.execute(Uuid::new_v4()).await;

        assert!(matches!(
            res.err(),
            Some(DeleteFlashcardError::PersistenceError(_))
        ));
    }
}
