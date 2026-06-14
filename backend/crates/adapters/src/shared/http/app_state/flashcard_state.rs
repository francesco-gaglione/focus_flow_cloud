use application::flashcards::use_cases::create_flashcards::CreateFlashcardUseCase;
use application::flashcards::use_cases::create_folder::CreateFolderUseCase;
use application::flashcards::use_cases::delete_flashcard::DeleteFlashcardUseCase;
use application::flashcards::use_cases::delete_folder::DeleteFolderUseCase;
use application::flashcards::use_cases::get_due_flashcards::GetDueFlashcardsUseCase;
use application::flashcards::use_cases::get_flashcard::GetFlashcardUseCase;
use application::flashcards::use_cases::get_folder_contents::GetFolderContentsUseCase;
use application::flashcards::use_cases::review_flashcard::ReviewFlashcardUseCase;
use application::flashcards::use_cases::update_flashcard::UpdateFlashcardUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct FlashcardState {
    pub create_flashcard_uc: Arc<CreateFlashcardUseCase>,
    pub get_folder_contents_uc: Arc<GetFolderContentsUseCase>,
    pub get_flashcard_uc: Arc<GetFlashcardUseCase>,
    pub update_flashcard_uc: Arc<UpdateFlashcardUseCase>,
    pub delete_flashcard_uc: Arc<DeleteFlashcardUseCase>,
    pub review_flashcard_uc: Arc<ReviewFlashcardUseCase>,
    pub get_due_flashcards_uc: Arc<GetDueFlashcardsUseCase>,
    pub create_folder_uc: Arc<CreateFolderUseCase>,
    pub delete_folder_uc: Arc<DeleteFolderUseCase>,
}
