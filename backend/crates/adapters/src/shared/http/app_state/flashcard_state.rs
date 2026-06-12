use application::flashcards::use_cases::create_flashcards::CreateFlashcardUseCase;
use application::flashcards::use_cases::get_folder_contents::GetFolderContentsUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct FlashcardState {
    pub create_flashcard_uc: Arc<CreateFlashcardUseCase>,
    pub get_folder_contents_uc: Arc<GetFolderContentsUseCase>,
}
