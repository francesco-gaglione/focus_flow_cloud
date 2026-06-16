use crate::shared::traits::persistence_error::PersistenceResult;
use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct ActivityEntry {
    pub date: NaiveDate,
    pub count: i64,
}

pub struct GlobalFlashcardStats {
    pub total_cards: i64,
    pub due_today: i64,
    pub reviewed_today: i64,
    pub retention_rate_30d: f64,
}

pub struct FolderFlashcardStats {
    pub total_cards: i64,
    pub due_today: i64,
    pub reviewed_today: i64,
}

#[async_trait]
pub trait FlashcardStatsPersistence: Send + Sync {
    async fn get_global_stats(&self, user_id: &Uuid) -> PersistenceResult<GlobalFlashcardStats>;
    async fn get_folder_stats(
        &self,
        folder_id: &Uuid,
        user_id: &Uuid,
    ) -> PersistenceResult<FolderFlashcardStats>;
    async fn get_activity_heatmap(
        &self,
        user_id: &Uuid,
        days: u32,
    ) -> PersistenceResult<Vec<ActivityEntry>>;
}
