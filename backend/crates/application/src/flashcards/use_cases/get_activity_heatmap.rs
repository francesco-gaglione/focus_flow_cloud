use std::sync::Arc;

use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    flashcards::traits::flashcard_stats_persistence::{ActivityEntry, FlashcardStatsPersistence},
    shared::traits::persistence_error::PersistenceError,
};

#[derive(Debug, Error, Clone, PartialEq)]
pub enum GetActivityHeatmapError {
    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type GetActivityHeatmapResult<T> = Result<T, GetActivityHeatmapError>;

pub struct GetActivityHeatmapUseCase {
    stats_persistence: Arc<dyn FlashcardStatsPersistence>,
}

impl GetActivityHeatmapUseCase {
    pub fn new(stats_persistence: Arc<dyn FlashcardStatsPersistence>) -> Self {
        Self { stats_persistence }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        user_id: Uuid,
        days: u32,
    ) -> GetActivityHeatmapResult<Vec<ActivityEntry>> {
        let entries = self
            .stats_persistence
            .get_activity_heatmap(&user_id, days)
            .await?;
        Ok(entries)
    }
}
