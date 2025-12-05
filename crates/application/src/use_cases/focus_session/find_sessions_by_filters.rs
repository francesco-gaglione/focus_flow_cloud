use crate::app_error::{AppError, AppResult};
use crate::traits::focus_session_persistence::FocusSessionPersistence;
use crate::use_cases::focus_session::command::find_session_filters::FindSessionFiltersCommand;
use crate::use_cases::persistance_command::find_session_by_filters_data::FindSessionByFiltersData;
use chrono::DateTime;
use domain::entities::focus_session::FocusSession;
use std::sync::Arc;
use uuid::Uuid;

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
    ) -> AppResult<Vec<FocusSession>> {
        let start_date = match filters.date_range.as_ref() {
            Some(r) => Some(
                DateTime::from_timestamp(r.start_date, 0)
                    .ok_or_else(|| AppError::BadRequest("Invalid start date".to_string()))?,
            ),
            None => None,
        };

        let end_date = match filters.date_range.as_ref() {
            Some(r) => Some(
                DateTime::from_timestamp(r.end_date, 0)
                    .ok_or_else(|| AppError::BadRequest("Invalid end date".to_string()))?,
            ),
            None => None,
        };

        let category_ids = filters
            .category_ids
            .map(|ids| ids.iter().map(|id| Uuid::parse_str(id).unwrap()).collect());

        let min_concentration_score = filters.concentration_score_range.clone().map(|s| s.min);
        let max_concentration_score = filters.concentration_score_range.clone().map(|s| s.max);

        self.session_persistence
            .find_by_filters(FindSessionByFiltersData {
                start_date,
                end_date,
                category_ids,
                session_type: filters.session_type,
                min_concentration_score,
                max_concentration_score,
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::focus_session_persistence::MockFocusSessionPersistence;
    use crate::use_cases::focus_session::command::find_session_filters::{
        ConcentrationScoreFilter, FocusSessionDateFilter,
    };
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

        let focus_session = FocusSession::new_with_id(
            session_id,
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
            category_ids: Some(vec![category_id.to_string()]),
            session_type: Some(FocusSessionType::Work),
            concentration_score_range: Some(ConcentrationScoreFilter { min: 1, max: 5 }),
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
    async fn test_find_session_by_filters_empty_result() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        mock_session_persistence
            .expect_find_by_filters()
            .returning(|_| Ok(vec![]));

        let use_case = FindSessionsByFiltersUseCase::new(Arc::new(mock_session_persistence));

        let filters = FindSessionFiltersCommand {
            date_range: None,
            category_ids: None,
            session_type: None,
            concentration_score_range: None,
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
            date_range: Some(FocusSessionDateFilter {
                start_date: i64::MAX, // Invalid timestamp
                end_date: 1761119000,
            }),
            category_ids: None,
            session_type: None,
            concentration_score_range: None,
        };

        let use_case = FindSessionsByFiltersUseCase::new(Arc::new(mock_session_persistence));

        let result = use_case.execute(filters).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert_eq!(msg, "Invalid start date"),
            _ => panic!("Expected BadRequest for invalid start date"),
        }
    }
}
