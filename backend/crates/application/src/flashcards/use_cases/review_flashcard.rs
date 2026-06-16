use std::sync::Arc;

use chrono::{Duration, Utc};
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    flashcards::traits::{
        flashcard_persistence::FlashcardPersistence, spaced_repetition_port::SpacedRepetitionPort,
    },
    shared::traits::persistence_error::PersistenceError,
};

fn rating_to_str(rating: &CardRatingCommand) -> &'static str {
    match rating {
        CardRatingCommand::Again => "Again",
        CardRatingCommand::Hard => "Hard",
        CardRatingCommand::Good => "Good",
        CardRatingCommand::Easy => "Easy",
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ReviewFlashcardError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type ReviewFlashcardResult<T> = Result<T, ReviewFlashcardError>;

#[derive(Debug)]
pub struct ReviewFlashcardCommand {
    pub flashcard_id: Uuid,
    pub rating: CardRatingCommand,
    pub elapsed_days: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CardRatingCommand {
    Again,
    Hard,
    Good,
    Easy,
}

pub struct ReviewFlashcardUseCase {
    flashcard_persistence: Arc<dyn FlashcardPersistence>,
    srs: Arc<dyn SpacedRepetitionPort>,
}

impl ReviewFlashcardUseCase {
    pub fn new(
        flashcard_persistence: Arc<dyn FlashcardPersistence>,
        srs: Arc<dyn SpacedRepetitionPort>,
    ) -> Self {
        Self {
            flashcard_persistence,
            srs,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: ReviewFlashcardCommand) -> ReviewFlashcardResult<()> {
        let mut flashcard = self
            .flashcard_persistence
            .find_by_id(command.flashcard_id)
            .await?;

        let user_id = *flashcard.user_id();

        let current_memory_state = {
            let ms = flashcard.memory_state();
            if ms.stability() == 0.0 && ms.difficulty() == 0.0 {
                None
            } else {
                Some(ms.clone())
            }
        };

        let next_states = self
            .srs
            .next_states(current_memory_state, command.elapsed_days);

        let chosen = match command.rating {
            CardRatingCommand::Again => next_states.again,
            CardRatingCommand::Hard => next_states.hard,
            CardRatingCommand::Good => next_states.good,
            CardRatingCommand::Easy => next_states.easy,
        };

        let now = Utc::now();
        flashcard.update_memory_state(chosen.memory_state);
        flashcard.update_due_date(now + Duration::days(chosen.interval_days as i64));

        self.flashcard_persistence.update(&flashcard).await?;

        let rating_str = rating_to_str(&command.rating);
        self.flashcard_persistence
            .save_review(command.flashcard_id, user_id, rating_str, now)
            .await?;

        tracing::info!(
            "Flashcard reviewed id={} rating={:?} interval_days={}",
            command.flashcard_id,
            command.rating,
            chosen.interval_days
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use domain::flashcards::{
        entities::flashcard::Flashcard, value_objects::memory_state::MemoryState,
    };
    use uuid::Uuid;

    use crate::{
        flashcards::{
            traits::{
                flashcard_persistence::MockFlashcardPersistence,
                spaced_repetition_port::{SpacedRepetitionPort, SrsCardState, SrsNextStates},
            },
            use_cases::review_flashcard::{
                CardRatingCommand, ReviewFlashcardCommand, ReviewFlashcardUseCase,
            },
        },
        shared::traits::persistence_error::PersistenceError,
    };

    struct MockSrs;

    impl SpacedRepetitionPort for MockSrs {
        fn next_states(
            &self,
            _memory_state: Option<MemoryState>,
            _elapsed_days: u32,
        ) -> SrsNextStates {
            let state = SrsCardState {
                memory_state: MemoryState::new(2.5, 0.3),
                interval_days: 3,
            };
            SrsNextStates {
                again: state.clone(),
                hard: state.clone(),
                good: state.clone(),
                easy: state.clone(),
            }
        }
    }

    fn make_flashcard() -> Flashcard {
        Flashcard::new("front", "back", Uuid::new_v4())
    }

    #[tokio::test]
    async fn test_review_good_updates_flashcard() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence
            .expect_find_by_id()
            .times(1)
            .returning(|_| Ok(make_flashcard()));

        mock_persistence
            .expect_update()
            .times(1)
            .returning(|_| Ok(()));

        mock_persistence
            .expect_save_review()
            .times(1)
            .returning(|_, _, _, _| Ok(()));

        let uc = ReviewFlashcardUseCase::new(Arc::new(mock_persistence), Arc::new(MockSrs));

        let res = uc
            .execute(ReviewFlashcardCommand {
                flashcard_id: Uuid::new_v4(),
                rating: CardRatingCommand::Good,
                elapsed_days: 0,
            })
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_review_flashcard_not_found() {
        let mut mock_persistence = MockFlashcardPersistence::new();

        mock_persistence
            .expect_find_by_id()
            .times(1)
            .returning(|_| Err(PersistenceError::NotFound("not found".to_string())));

        let uc = ReviewFlashcardUseCase::new(Arc::new(mock_persistence), Arc::new(MockSrs));

        let res = uc
            .execute(ReviewFlashcardCommand {
                flashcard_id: Uuid::new_v4(),
                rating: CardRatingCommand::Again,
                elapsed_days: 0,
            })
            .await;

        assert!(matches!(
            res.err(),
            Some(crate::flashcards::use_cases::review_flashcard::ReviewFlashcardError::PersistenceError(_))
        ));
    }

    #[tokio::test]
    async fn test_review_all_ratings() {
        for rating in [
            CardRatingCommand::Again,
            CardRatingCommand::Hard,
            CardRatingCommand::Good,
            CardRatingCommand::Easy,
        ] {
            let mut mock_persistence = MockFlashcardPersistence::new();
            mock_persistence
                .expect_find_by_id()
                .times(1)
                .returning(|_| Ok(make_flashcard()));
            mock_persistence
                .expect_update()
                .times(1)
                .returning(|_| Ok(()));

            mock_persistence
                .expect_save_review()
                .times(1)
                .returning(|_, _, _, _| Ok(()));

            let uc = ReviewFlashcardUseCase::new(Arc::new(mock_persistence), Arc::new(MockSrs));

            let res = uc
                .execute(ReviewFlashcardCommand {
                    flashcard_id: Uuid::new_v4(),
                    rating,
                    elapsed_days: 1,
                })
                .await;

            assert!(res.is_ok());
        }
    }
}
