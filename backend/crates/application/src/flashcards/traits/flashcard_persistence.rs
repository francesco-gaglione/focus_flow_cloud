use crate::shared::traits::persistence_error::PersistenceResult;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::flashcards::entities::flashcard::Flashcard;
use domain::shared::entities::folder_metadata::FolderMetadata;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FlashcardPersistence: Send + Sync {
    async fn save(&self, flashcard: &Flashcard) -> PersistenceResult<()>;

    async fn save_to_folder(
        &self,
        flashcard: &Flashcard,
        folder_id: &Uuid,
    ) -> PersistenceResult<()>;

    async fn remove_from_folder(
        &self,
        flashcard: &Flashcard,
        folder_id: &Uuid,
    ) -> PersistenceResult<()>;

    async fn find_by_folder(&self, folder_id: &Uuid) -> PersistenceResult<Vec<Flashcard>>;

    async fn update(&self, flashcard: &Flashcard) -> PersistenceResult<()>;

    async fn find_by_id(&self, id: Uuid) -> PersistenceResult<Flashcard>;

    async fn delete(&self, id: Uuid) -> PersistenceResult<()>;

    async fn find_subfolders_by_parent(
        &self,
        parent_id: &Uuid,
    ) -> PersistenceResult<Vec<FolderMetadata>>;

    async fn find_root_folder_by_user(&self, user_id: &Uuid) -> PersistenceResult<FolderMetadata>;

    async fn create_root_folder_for_user(
        &self,
        user_id: &Uuid,
    ) -> PersistenceResult<FolderMetadata>;

    async fn find_due_by_user(&self, user_id: &Uuid) -> PersistenceResult<Vec<Flashcard>>;

    async fn create_folder(
        &self,
        name: &str,
        user_id: &Uuid,
        parent_id: &Uuid,
    ) -> PersistenceResult<FolderMetadata>;

    async fn delete_folder(&self, id: Uuid) -> PersistenceResult<()>;

    async fn save_review(
        &self,
        flashcard_id: Uuid,
        user_id: Uuid,
        rating: &str,
        reviewed_at: DateTime<Utc>,
    ) -> PersistenceResult<()>;

    async fn find_due_by_folder(&self, folder_id: &Uuid) -> PersistenceResult<Vec<Flashcard>>;
}
