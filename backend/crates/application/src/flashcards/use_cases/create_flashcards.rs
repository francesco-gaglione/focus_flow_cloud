use crate::{
    flashcards::traits::flashcard_persistence::FlashcardPersistence,
    shared::traits::persistence_error::PersistenceError,
};
use domain::flashcards::entities::flashcard::Flashcard;
use std::sync::Arc;
use thiserror::Error;
use tracing::{instrument, warn};
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateFlashcardCommand {
    pub front: String,
    pub back: String,
    pub user_id: Uuid,
    pub folder_id: Uuid,
}

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
    pub async fn execute(&self, command: CreateFlashcardCommand) -> CreateFlashcardResult<()> {
        if command.front.is_empty() {
            warn!("Invalid front side for user_id={}", command.user_id);
            return Err(CreateFlashcardError::InvalidFrontSideCard);
        }
        if command.back.is_empty() {
            warn!("Invalid back side for user_id={}", command.user_id);
            return Err(CreateFlashcardError::InvalidBackSideCard);
        }

        let flashcard = Flashcard::new(&command.front, &command.back, command.user_id);
        self.flashcard_persistence
            .save_to_folder(&flashcard, &command.folder_id)
            .await?;

        tracing::info!(
            "Flashcard created id={} user_id={} folder_id={}",
            flashcard.id(),
            command.user_id,
            command.folder_id
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests_create_flashcard_use_case {
    use std::sync::Arc;

    use uuid::Uuid;

    use crate::flashcards::{
        traits::flashcard_persistence::MockFlashcardPersistence,
        use_cases::create_flashcards::{CreateFlashcardCommand, CreateFlashcardUseCase},
    };

    fn make_command(front: &str, back: &str) -> CreateFlashcardCommand {
        CreateFlashcardCommand {
            front: front.to_string(),
            back: back.to_string(),
            user_id: Uuid::new_v4(),
            folder_id: Uuid::new_v4(),
        }
    }

    #[tokio::test]
    async fn create_flashcard() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence
            .expect_save_to_folder()
            .times(1)
            .returning(|_, _| Ok(()));

        let use_case = CreateFlashcardUseCase::new(Arc::new(mock_persistence));

        let res = use_case.execute(make_command("front", "back")).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn invalid_front_side_card() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence.expect_save_to_folder().times(0);

        let use_case = CreateFlashcardUseCase::new(Arc::new(mock_persistence));

        let res = use_case.execute(make_command("", "back")).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn invalid_back_side_card() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence.expect_save_to_folder().times(0);

        let use_case = CreateFlashcardUseCase::new(Arc::new(mock_persistence));

        let res = use_case.execute(make_command("front", "")).await;
        assert!(res.is_err());
    }
}
