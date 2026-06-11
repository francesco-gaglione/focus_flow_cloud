use adapters::shared::http::app_state::flashcard_state::FlashcardState;
use adapters::shared::persistence::PostgresPersistence;
use application::flashcards::use_cases::create_flashcards::CreateFlashcardUseCase;
use std::sync::Arc;

pub fn build_flashcard_state(postgres: Arc<PostgresPersistence>) -> FlashcardState {
    FlashcardState {
        create_flashcard_uc: Arc::new(CreateFlashcardUseCase::new(postgres.clone())),
    }
}
