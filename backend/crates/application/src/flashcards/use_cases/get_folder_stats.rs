use std::sync::Arc;

use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    flashcards::traits::flashcard_stats_persistence::{
        FlashcardStatsPersistence, FolderFlashcardStats,
    },
    shared::traits::persistence_error::PersistenceError,
};

#[derive(Debug, Error, Clone, PartialEq)]
pub enum GetFolderStatsError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetFolderStatsResult<T> = Result<T, GetFolderStatsError>;

pub struct GetFolderStatsUseCase {
    stats_persistence: Arc<dyn FlashcardStatsPersistence>,
}

impl GetFolderStatsUseCase {
    pub fn new(stats_persistence: Arc<dyn FlashcardStatsPersistence>) -> Self {
        Self { stats_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        folder_id: Uuid,
        user_id: Uuid,
    ) -> GetFolderStatsResult<FolderFlashcardStats> {
        let stats = self
            .stats_persistence
            .get_folder_stats(&folder_id, &user_id)
            .await?;
        Ok(stats)
    }
}
