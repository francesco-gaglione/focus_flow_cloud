use crate::shared::traits::persistence_error::PersistenceResult;
use async_trait::async_trait;
use domain::flashcards::entities::flashcard::Flashcard;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FlashcardPersistence: Send + Sync {
    async fn save(&self, flashcard: &Flashcard) -> PersistenceResult<()>;

    async fn update(&self, flashcard: &Flashcard) -> PersistenceResult<()>;

    async fn find_by_id(&self, id: Uuid) -> PersistenceResult<Flashcard>;

    async fn delete(&self, id: Uuid) -> PersistenceResult<()>;
}
