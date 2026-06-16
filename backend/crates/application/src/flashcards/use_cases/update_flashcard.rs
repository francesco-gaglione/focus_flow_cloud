use std::sync::Arc;

use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    flashcards::traits::flashcard_persistence::FlashcardPersistence,
    shared::traits::persistence_error::PersistenceError,
};

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum UpdateFlashcardError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("nothing to do")]
    NothingToDo,
}

pub type UpdateFlashcardResult<T> = Result<T, UpdateFlashcardError>;

#[derive(Debug, Clone, Default)]
pub struct UpdateFlashcardCommand {
    pub card_id: Uuid,
    pub front: Option<String>,
    pub back: Option<String>,
}

pub struct UpdateFlashcardUseCase {
    flashcard_persistence: Arc<dyn FlashcardPersistence>,
}

impl UpdateFlashcardUseCase {
    pub fn new(flashcard_persistence: Arc<dyn FlashcardPersistence>) -> Self {
        Self {
            flashcard_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: UpdateFlashcardCommand) -> UpdateFlashcardResult<()> {
        if command.front.is_none() && command.back.is_none() {
            tracing::warn!(
                "Update flashcard called with nothing to do id={}",
                command.card_id
            );
            return Err(UpdateFlashcardError::NothingToDo);
        }

        let mut flashcard = self
            .flashcard_persistence
            .find_by_id(command.card_id)
            .await?;

        if let Some(front) = command.front {
            flashcard.update_front(&front);
        }

        if let Some(back) = command.back {
            flashcard.update_back(&back);
        }

        self.flashcard_persistence.update(&flashcard).await?;
        tracing::info!("Flashcard updated id={}", command.card_id);
        Ok(())
    }
}

#[cfg(test)]
pub mod tests_update_flashcard_usecase {
    use std::sync::Arc;

    use domain::flashcards::entities::flashcard::Flashcard;
    use uuid::Uuid;

    use crate::flashcards::{
        traits::flashcard_persistence::MockFlashcardPersistence,
        use_cases::update_flashcard::{
            UpdateFlashcardCommand, UpdateFlashcardError, UpdateFlashcardUseCase,
        },
    };

    fn make_flashcard() -> Flashcard {
        Flashcard::new("front", "back", Uuid::new_v4())
    }

    #[tokio::test]
    async fn test_update_flashcard_front() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence
            .expect_find_by_id()
            .times(1)
            .returning(|_| Ok(make_flashcard()));

        mock_persistence
            .expect_update()
            .times(1)
            .returning(|_| Ok(()));

        let uc = UpdateFlashcardUseCase::new(Arc::new(mock_persistence));

        let res = uc
            .execute(UpdateFlashcardCommand {
                card_id: Uuid::new_v4(),
                front: Some("new front".to_string()),
                back: None,
            })
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_update_flashcard_back() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence
            .expect_find_by_id()
            .times(1)
            .returning(|_| Ok(make_flashcard()));

        mock_persistence
            .expect_update()
            .times(1)
            .returning(|_| Ok(()));

        let uc = UpdateFlashcardUseCase::new(Arc::new(mock_persistence));

        let res = uc
            .execute(UpdateFlashcardCommand {
                card_id: Uuid::new_v4(),
                front: None,
                back: Some("new back".to_string()),
            })
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_update_flashcard_nothing_to_do() {
        let mock_persistence = MockFlashcardPersistence::new();

        let uc = UpdateFlashcardUseCase::new(Arc::new(mock_persistence));

        let res = uc
            .execute(UpdateFlashcardCommand {
                card_id: Uuid::new_v4(),
                front: None,
                back: None,
            })
            .await;

        assert_eq!(res.err(), Some(UpdateFlashcardError::NothingToDo));
    }

    #[tokio::test]
    async fn test_update_flashcard_not_found() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence
            .expect_find_by_id()
            .times(1)
            .returning(|_| {
                Err(
                    crate::shared::traits::persistence_error::PersistenceError::NotFound(
                        "not found".to_string(),
                    ),
                )
            });

        let uc = UpdateFlashcardUseCase::new(Arc::new(mock_persistence));

        let res = uc
            .execute(UpdateFlashcardCommand {
                card_id: Uuid::new_v4(),
                front: Some("x".to_string()),
                back: None,
            })
            .await;

        assert!(matches!(
            res.err(),
            Some(UpdateFlashcardError::PersistenceError(_))
        ));
    }
}
