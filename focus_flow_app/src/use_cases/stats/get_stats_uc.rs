use std::collections::HashMap;

use shared::stats::GetStatsResponseDto;

use crate::clients::{
    category_http_client::get_all_categories, http_client::ApiError, stats_client::get_stats,
};

#[derive(Debug, thiserror::Error)]
pub enum StatsError {
    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),
}

#[derive(Clone, Default)]
pub struct StatsData {
    pub stats: GetStatsResponseDto,
    pub category_colors: HashMap<String, String>,
}

pub async fn get_stats_uc() -> Result<StatsData, StatsError> {
    let stats = get_stats().await?;
    let category_colors: HashMap<String, String> = get_all_categories()
        .await
        .map(|r| r.categories.into_iter().map(|c| (c.id, c.color)).collect())
        .unwrap_or_default();
    Ok(StatsData { stats, category_colors })
}
