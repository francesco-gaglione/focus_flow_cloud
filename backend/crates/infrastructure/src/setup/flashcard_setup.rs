use adapters::flashcards::persistence::fsrs_adapter::FsrsAdapter;
use adapters::shared::http::app_state::flashcard_state::FlashcardState;
use adapters::shared::persistence::PostgresPersistence;
use application::flashcards::use_cases::create_flashcards::CreateFlashcardUseCase;
use application::flashcards::use_cases::create_folder::CreateFolderUseCase;
use application::flashcards::use_cases::delete_flashcard::DeleteFlashcardUseCase;
use application::flashcards::use_cases::delete_folder::DeleteFolderUseCase;
use application::flashcards::use_cases::get_activity_heatmap::GetActivityHeatmapUseCase;
use application::flashcards::use_cases::get_due_flashcards::GetDueFlashcardsUseCase;
use application::flashcards::use_cases::get_flashcard::GetFlashcardUseCase;
use application::flashcards::use_cases::get_folder_contents::GetFolderContentsUseCase;
use application::flashcards::use_cases::get_folder_review_queue::GetFolderReviewQueueUseCase;
use application::flashcards::use_cases::get_folder_stats::GetFolderStatsUseCase;
use application::flashcards::use_cases::get_global_stats::GetGlobalStatsUseCase;
use application::flashcards::use_cases::review_flashcard::ReviewFlashcardUseCase;
use application::flashcards::use_cases::update_flashcard::UpdateFlashcardUseCase;
use std::sync::Arc;

pub fn build_flashcard_state(postgres: Arc<PostgresPersistence>) -> FlashcardState {
    let fsrs = Arc::new(FsrsAdapter::default());

    FlashcardState {
        create_flashcard_uc: Arc::new(CreateFlashcardUseCase::new(postgres.clone())),
        get_folder_contents_uc: Arc::new(GetFolderContentsUseCase::new(postgres.clone())),
        get_flashcard_uc: Arc::new(GetFlashcardUseCase::new(postgres.clone())),
        update_flashcard_uc: Arc::new(UpdateFlashcardUseCase::new(postgres.clone())),
        delete_flashcard_uc: Arc::new(DeleteFlashcardUseCase::new(postgres.clone())),
        review_flashcard_uc: Arc::new(ReviewFlashcardUseCase::new(postgres.clone(), fsrs)),
        get_due_flashcards_uc: Arc::new(GetDueFlashcardsUseCase::new(postgres.clone())),
        create_folder_uc: Arc::new(CreateFolderUseCase::new(postgres.clone())),
        delete_folder_uc: Arc::new(DeleteFolderUseCase::new(postgres.clone())),
        get_global_stats_uc: Arc::new(GetGlobalStatsUseCase::new(postgres.clone())),
        get_folder_stats_uc: Arc::new(GetFolderStatsUseCase::new(postgres.clone())),
        get_activity_heatmap_uc: Arc::new(GetActivityHeatmapUseCase::new(postgres.clone())),
        get_folder_review_queue_uc: Arc::new(GetFolderReviewQueueUseCase::new(postgres.clone())),
    }
}
