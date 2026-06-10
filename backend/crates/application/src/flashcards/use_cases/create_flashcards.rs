use crate::{
    flashcards::traits::flashcard_persistence::FlashcardPersistence,
    shared::traits::persistence_error::PersistenceError,
};
use domain::flashcards::entities::flashcard::Flashcard;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CreateFlashcardError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("invalid front side card")]
    InvalidFrontSideCard,

    #[error("invalid back side card")]
    InvalidBackSideCard,
}

pub type CreateFlashcardResult<T> = Result<T, CreateFlashcardError>;

pub struct CreateFlashcardUseCase {
    pub flashcard_persistence: Arc<dyn FlashcardPersistence>,
}

impl CreateFlashcardUseCase {
    pub fn new(flashcard_persistence: Arc<dyn FlashcardPersistence>) -> Self {
        Self {
            flashcard_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        front: &str,
        back: &str,
        user_id: Uuid,
    ) -> CreateFlashcardResult<()> {
        if front.is_empty() {
            return Err(CreateFlashcardError::InvalidFrontSideCard);
        }
        if back.is_empty() {
            return Err(CreateFlashcardError::InvalidBackSideCard);
        }

        let flashcard = Flashcard::new(front, back, user_id);
        self.flashcard_persistence.save(&flashcard).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests_create_flashcard_use_case {
    use std::sync::Arc;

    use uuid::Uuid;

    use crate::flashcards::{
        traits::flashcard_persistence::MockFlashcardPersistence,
        use_cases::create_flashcards::CreateFlashcardUseCase,
    };

    #[tokio::test]
    async fn create_flashcard() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence
            .expect_save()
            .times(1)
            .returning(|_| Ok(()));

        let use_case = CreateFlashcardUseCase::new(Arc::new(mock_persistence));
        let user_id = Uuid::new_v4();

        let res = use_case.execute("front", "back", user_id).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn invalid_front_side_card() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence.expect_save().times(0);

        let use_case = CreateFlashcardUseCase::new(Arc::new(mock_persistence));
        let user_id = Uuid::new_v4();

        let res = use_case.execute("", "back", user_id).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn invalid_back_side_card() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence.expect_save().times(0);

        let use_case = CreateFlashcardUseCase::new(Arc::new(mock_persistence));
        let user_id = Uuid::new_v4();

        let res = use_case.execute("front", "", user_id).await;
        assert!(res.is_err());
    }
}
