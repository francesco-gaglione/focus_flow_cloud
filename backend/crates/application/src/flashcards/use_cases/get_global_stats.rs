use std::sync::Arc;

use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    flashcards::traits::flashcard_stats_persistence::{
        FlashcardStatsPersistence, GlobalFlashcardStats,
    },
    shared::traits::persistence_error::PersistenceError,
};

#[derive(Debug, Error, Clone, PartialEq)]
pub enum GetGlobalStatsError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetGlobalStatsResult<T> = Result<T, GetGlobalStatsError>;

pub struct GetGlobalStatsUseCase {
    stats_persistence: Arc<dyn FlashcardStatsPersistence>,
}

impl GetGlobalStatsUseCase {
    pub fn new(stats_persistence: Arc<dyn FlashcardStatsPersistence>) -> Self {
        Self { stats_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, user_id: Uuid) -> GetGlobalStatsResult<GlobalFlashcardStats> {
        let stats = self.stats_persistence.get_global_stats(&user_id).await?;
        Ok(stats)
    }
}
