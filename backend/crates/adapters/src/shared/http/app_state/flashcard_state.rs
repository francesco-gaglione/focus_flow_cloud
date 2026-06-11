use std::sync::Arc;

use application::flashcards::use_cases::create_flashcards::CreateFlashcardUseCase;

#[derive(Clone)]
pub struct FlashcardState {
    pub create_flashcard_uc: Arc<CreateFlashcardUseCase>,
}
