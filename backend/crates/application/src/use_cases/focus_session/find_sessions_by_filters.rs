use chrono::DateTime;
use domain::entities::focus_session::{FocusSession, SessionFilter};
use domain::entities::focus_session_type::FocusSessionType;
use domain::error::persistence_error::PersistenceError;
use domain::traits::focus_session_persistence::FocusSessionPersistence;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum FindSessionByFiltersError {
    #[error("Invalid date range: {0}")]
    InvalidDateRange(String),
    #[error("Invalid category id")]
    InvalidCategoryId,
    #[error("Invalid task id")]
    InvalidTaskId,
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
}

pub type FindSessionByFiltersResult<T> = Result<T, FindSessionByFiltersError>;

#[derive(Debug, Clone)]
pub struct FindSessionFiltersCommand {
    pub user_id: Uuid,
    pub date_range: Option<FocusSessionDateFilter>,
    pub category_ids: Option<Vec<String>>,
    pub task_ids: Option<Vec<String>>,
    pub session_type: Option<FocusSessionType>,
    pub concentration_score_range: Option<ConcentrationScoreFilter>,
    pub has_notes: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct FocusSessionDateFilter {
    pub start_date: i64,
    pub end_date: i64,
}

#[derive(Debug, Clone)]
pub struct ConcentrationScoreFilter {
    pub min: i32,
    pub max: i32,
}

pub struct FindSessionsByFiltersUseCase {
    session_persistence: Arc<dyn FocusSessionPersistence>,
}

impl FindSessionsByFiltersUseCase {
    pub fn new(session_persistence: Arc<dyn FocusSessionPersistence>) -> Self {
        Self {
            session_persistence,
        }
    }

    pub async fn execute(
        &self,
        filters: FindSessionFiltersCommand,
    ) -> FindSessionByFiltersResult<Vec<FocusSession>> {
        let (start_date, end_date) = filters
            .date_range
            .as_ref()
            .map(|r| {
                let start = DateTime::from_timestamp(r.start_date, 0).ok_or_else(|| {
                    FindSessionByFiltersError::InvalidDateRange(r.start_date.to_string())
                });
                let end = DateTime::from_timestamp(r.end_date, 0).ok_or_else(|| {
                    FindSessionByFiltersError::InvalidDateRange(r.end_date.to_string())
                });
                start.and_then(|s| end.map(|e| (Some(s), Some(e))))
            })
            .transpose()?
            .unwrap_or((None, None));

        let category_ids = filters
            .category_ids
            .map(|ids| {
                ids.iter()
                    .map(|id| Uuid::parse_str(id))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| FindSessionByFiltersError::InvalidCategoryId)
            })
            .transpose()?;

        let task_ids = filters
            .task_ids
            .map(|ids| {
                ids.iter()
                    .map(|id| Uuid::parse_str(id))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| FindSessionByFiltersError::InvalidTaskId)
            })
            .transpose()?;

        let (min_concentration_score, max_concentration_score) = filters
            .concentration_score_range
            .map(|s| (s.min, s.max))
            .unzip();

        let filter = SessionFilter {
            user_id: filters.user_id,
            start_date,
            end_date,
            category_ids,
            task_ids,
            session_type: filters.session_type,
            min_concentration_score,
            max_concentration_score,
            has_notes: filters.has_notes,
        };

        Ok(self.session_persistence.find_by_filters(filter).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::MockFocusSessionPersistence;
    use chrono::DateTime;
    use domain::entities::focus_session_type::FocusSessionType;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_find_session_by_filters_success() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        let category_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let started_at = DateTime::from_timestamp(1761118663, 0).unwrap();
        let ended_at = DateTime::from_timestamp(1761118714, 0).unwrap();

        let focus_session = FocusSession::reconstitute(
            session_id,
            Uuid::new_v4(),
            Some(category_id),
            Some(task_id),
            FocusSessionType::Work,
            Some(51),
            Some(5),
            Some("note".to_string()),
            started_at,
            Some(ended_at),
            started_at,
        );

        mock_session_persistence
            .expect_find_by_filters()
            .returning(move |_| Ok(vec![focus_session.clone()]));

        let filters = FindSessionFiltersCommand {
            date_range: Some(FocusSessionDateFilter {
                start_date: 1761118000,
                end_date: 1761119000,
            }),
            user_id: Uuid::new_v4(),
            category_ids: Some(vec![category_id.to_string()]),
            task_ids: None,
            session_type: Some(FocusSessionType::Work),
            concentration_score_range: Some(ConcentrationScoreFilter { min: 1, max: 5 }),
            has_notes: None,
        };

        let use_case = FindSessionsByFiltersUseCase::new(Arc::new(mock_session_persistence));

        let result = use_case.execute(filters).await;
        assert!(result.is_ok());
        let sessions = result.unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id(), session_id);
        assert_eq!(sessions[0].concentration_score(), Some(5));
    }

    #[tokio::test]
    async fn test_find_session_by_filters_has_notes() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        let session_id = Uuid::new_v4();
        let started_at = DateTime::from_timestamp(1761118663, 0).unwrap();
        let ended_at = DateTime::from_timestamp(1761118714, 0).unwrap();

        let focus_session = FocusSession::reconstitute(
            session_id,
            Uuid::new_v4(),
            None,
            None,
            FocusSessionType::Work,
            Some(51),
            Some(5),
            Some("note".to_string()),
            started_at,
            Some(ended_at),
            started_at,
        );

        mock_session_persistence
            .expect_find_by_filters()
            .returning(move |filter| {
                if filter.has_notes == Some(true) {
                    Ok(vec![focus_session.clone()])
                } else {
                    Ok(vec![])
                }
            });

        let use_case = FindSessionsByFiltersUseCase::new(Arc::new(mock_session_persistence));

        let filters = FindSessionFiltersCommand {
            user_id: Uuid::new_v4(),
            date_range: None,
            category_ids: None,
            task_ids: None,
            session_type: None,
            concentration_score_range: None,
            has_notes: Some(true),
        };

        let result = use_case.execute(filters).await;
        assert!(result.is_ok());
        let sessions = result.unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id(), session_id);
    }

    #[tokio::test]
    async fn test_find_session_by_filters_empty_result() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        mock_session_persistence
            .expect_find_by_filters()
            .returning(|_| Ok(vec![]));

        let use_case = FindSessionsByFiltersUseCase::new(Arc::new(mock_session_persistence));

        let filters = FindSessionFiltersCommand {
            user_id: Uuid::new_v4(),
            date_range: None,
            category_ids: None,
            task_ids: None,
            session_type: None,
            concentration_score_range: None,
            has_notes: None,
        };

        let result = use_case.execute(filters).await;
        assert!(result.is_ok());
        let sessions = result.unwrap();
        assert_eq!(sessions.len(), 0);
    }

    #[tokio::test]
    async fn test_find_session_by_filters_invalid_date_error() {
        let mock_session_persistence = MockFocusSessionPersistence::new();

        // Test invalid start date (would cause DateTime::from_timestamp_secs to return None)
        let filters = FindSessionFiltersCommand {
            user_id: Uuid::new_v4(),
            date_range: Some(FocusSessionDateFilter {
                start_date: i64::MAX, // Invalid timestamp
                end_date: 1761119000,
            }),
            category_ids: None,
            task_ids: None,
            session_type: None,
            concentration_score_range: None,
            has_notes: None,
        };

        let use_case = FindSessionsByFiltersUseCase::new(Arc::new(mock_session_persistence));

        let result = use_case.execute(filters).await;
        assert!(result.is_err());
        matches!(
            result.unwrap_err(),
            FindSessionByFiltersError::InvalidDateRange(_)
        );
    }
}
