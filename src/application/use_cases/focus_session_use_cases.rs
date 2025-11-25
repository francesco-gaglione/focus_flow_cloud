use chrono::DateTime;
use uuid::Uuid;

use crate::application::app_error::{AppError, AppResult};
use crate::application::traits::FocusSessionPersistence;
use crate::application::use_cases::commands::create_foucs_session::CreateFocusSessionCommand;
use crate::application::use_cases::commands::create_manual_session::CreateManualFocusSessionCommand;
use crate::application::use_cases::commands::find_session_filters::FindSessionFiltersCommand;
use crate::application::use_cases::persistance_command::create_focus_session_data::CreateSessionData;
use crate::application::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData;
use crate::application::use_cases::persistance_command::find_session_by_filters_data::FindSessionByFiltersData;
use crate::domain::entities::focus_session::FocusSession;
use std::sync::Arc;

pub struct FocusSessionUseCases {
    session_persistence: Arc<dyn FocusSessionPersistence>,
}

impl FocusSessionUseCases {
    pub fn new(session_persistence: Arc<dyn FocusSessionPersistence>) -> Self {
        Self {
            session_persistence,
        }
    }

    pub async fn find_sessions_by_filters(
        &self,
        filters: FindSessionFiltersCommand,
    ) -> AppResult<Vec<FocusSession>> {
        let start_date = match filters.date_range.as_ref() {
            Some(r) => Some(
                DateTime::from_timestamp_secs(r.start_date)
                    .ok_or_else(|| AppError::BadRequest("Invalid start date".to_string()))?
                    .into(),
            ),
            None => None,
        };

        let end_date = match filters.date_range.as_ref() {
            Some(r) => Some(
                DateTime::from_timestamp_secs(r.end_date)
                    .ok_or_else(|| AppError::BadRequest("Invalid end date".to_string()))?
                    .into(),
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

    pub async fn create_manual_session(
        &self,
        session: CreateManualFocusSessionCommand,
    ) -> AppResult<FocusSession> {
        let duration = session.ended_at.timestamp() - session.started_at.timestamp();

        let manual_session_data = CreateManualSessionData {
            task_id: session.task_id,
            category_id: session.category_id,
            session_type: session.session_type.clone(),
            concentration_score: session.concentration_score,
            notes: session.notes.clone(),
            actual_duration: duration,
            started_at: session.started_at,
            ended_at: session.ended_at,
        };

        self.session_persistence
            .create_manual_session(&manual_session_data)
            .await
    }

    pub async fn create_session(
        &self,
        session: CreateFocusSessionCommand,
    ) -> AppResult<FocusSession> {
        let session_data = CreateSessionData {
            task_id: session.task_id,
            category_id: session.category_id,
            session_type: session.session_type.clone(),
            concentration_score: session.concentration_score,
            notes: session.notes.clone(),
            actual_duration: session.actual_duration,
            started_at: session.started_at,
            ended_at: session.ended_at,
        };

        self.session_persistence.create_session(session_data).await
    }
}

#[cfg(test)]
mod focus_session_use_cases_tests {
    use chrono::DateTime;
    use std::sync::Arc;
    use uuid::Uuid;

    use crate::{
        application::{
            app_error::AppError,
            traits::MockFocusSessionPersistence,
            use_cases::{
                commands::{
                    create_foucs_session::CreateFocusSessionCommand,
                    create_manual_session::CreateManualFocusSessionCommand,
                    find_session_filters::{
                        ConcentrationScoreFilter, FindSessionFiltersCommand, FocusSessionDateFilter,
                    },
                },
                focus_session_use_cases::FocusSessionUseCases,
            },
        },
        domain::entities::{focus_session::FocusSession, focus_session_type::FocusSessionType},
    };

    #[tokio::test]
    async fn test_create_session_success() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        let category_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let started_at = DateTime::from_timestamp(1761118663, 0).unwrap();
        let ended_at = DateTime::from_timestamp(1761118714, 0).unwrap();

        let focus_session = FocusSession::new_with_id(
            id,
            Some(category_id),
            Some(task_id),
            FocusSessionType::Work,
            Some(51),
            Some(3),
            Some("note".to_string()),
            started_at,
            Some(ended_at),
            started_at,
        );

        mock_session_persistence
            .expect_create_session()
            .returning(move |_| Ok(focus_session.clone()));

        let use_cases = FocusSessionUseCases::new(Arc::new(mock_session_persistence));

        let cmd = CreateFocusSessionCommand {
            category_id: Some(category_id),
            task_id: Some(task_id),
            session_type: FocusSessionType::Work,
            concentration_score: Some(3),
            actual_duration: 51,
            notes: Some("notes".to_string()),
            started_at,
            ended_at,
        };

        let result = use_cases.create_session(cmd).await;
        assert!(result.is_ok());
        let session = result.unwrap();
        assert_eq!(session.id(), id);
        assert_eq!(session.actual_duration(), Some(51));
    }

    #[tokio::test]
    async fn test_create_session_error() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        mock_session_persistence
            .expect_create_session()
            .returning(|_| Err(AppError::Database("Database error".to_string())));

        let use_cases = FocusSessionUseCases::new(Arc::new(mock_session_persistence));

        let cmd = CreateFocusSessionCommand {
            category_id: Some(Uuid::new_v4()),
            task_id: Some(Uuid::new_v4()),
            session_type: FocusSessionType::Work,
            concentration_score: Some(3),
            actual_duration: 1200,
            notes: Some("notes".to_string()),
            started_at: DateTime::from_timestamp(1761118663, 0).unwrap(),
            ended_at: DateTime::from_timestamp(1761118714, 0).unwrap(),
        };

        let result = use_cases.create_session(cmd).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Database(msg) => assert_eq!(msg, "Database error"),
            _ => panic!("Expected InternalServerError"),
        }
    }

    #[tokio::test]
    async fn test_create_manual_session_success() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        let category_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let started_at = DateTime::from_timestamp(1761118663, 0).unwrap();
        let ended_at = DateTime::from_timestamp(1761118714, 0).unwrap();
        let duration = ended_at.timestamp() - started_at.timestamp();

        let focus_session = FocusSession::new_with_id(
            id,
            Some(category_id),
            Some(task_id),
            FocusSessionType::Work,
            Some(duration),
            Some(4),
            Some("manual session notes".to_string()),
            started_at,
            Some(ended_at),
            started_at,
        );

        mock_session_persistence
            .expect_create_manual_session()
            .returning(move |_| Ok(focus_session.clone()));

        let use_cases = FocusSessionUseCases::new(Arc::new(mock_session_persistence));

        let cmd = CreateManualFocusSessionCommand {
            category_id: Some(category_id),
            task_id: Some(task_id),
            session_type: FocusSessionType::Work,
            concentration_score: Some(4),
            notes: Some("manual session notes".to_string()),
            started_at,
            ended_at,
        };

        let result = use_cases.create_manual_session(cmd).await;
        assert!(result.is_ok());
        let session = result.unwrap();
        assert_eq!(session.actual_duration(), Some(duration));
        assert_eq!(session.concentration_score(), Some(4));
    }

    #[tokio::test]
    async fn test_create_manual_session_error() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        mock_session_persistence
            .expect_create_manual_session()
            .returning(|_| Err(AppError::BadRequest("Invalid session data".to_string())));

        let use_cases = FocusSessionUseCases::new(Arc::new(mock_session_persistence));

        let cmd = CreateManualFocusSessionCommand {
            category_id: Some(Uuid::new_v4()),
            task_id: Some(Uuid::new_v4()),
            session_type: FocusSessionType::Work,
            concentration_score: Some(3),
            notes: Some("notes".to_string()),
            started_at: DateTime::from_timestamp(1761118663, 0).unwrap(),
            ended_at: DateTime::from_timestamp(1761118714, 0).unwrap(),
        };

        let result = use_cases.create_manual_session(cmd).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert_eq!(msg, "Invalid session data"),
            _ => panic!("Expected BadRequest"),
        }
    }

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

        let use_cases = FocusSessionUseCases::new(Arc::new(mock_session_persistence));

        let filters = FindSessionFiltersCommand {
            date_range: Some(FocusSessionDateFilter {
                start_date: 1761118000,
                end_date: 1761119000,
            }),
            category_ids: Some(vec![category_id.to_string()]),
            session_type: Some(FocusSessionType::Work),
            concentration_score_range: Some(ConcentrationScoreFilter { min: 1, max: 5 }),
        };

        let result = use_cases.find_sessions_by_filters(filters).await;
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

        let use_cases = FocusSessionUseCases::new(Arc::new(mock_session_persistence));

        let filters = FindSessionFiltersCommand {
            date_range: None,
            category_ids: None,
            session_type: None,
            concentration_score_range: None,
        };

        let result = use_cases.find_sessions_by_filters(filters).await;
        assert!(result.is_ok());
        let sessions = result.unwrap();
        assert_eq!(sessions.len(), 0);
    }

    #[tokio::test]
    async fn test_find_session_by_filters_invalid_date_error() {
        let mock_session_persistence = MockFocusSessionPersistence::new();
        let use_cases = FocusSessionUseCases::new(Arc::new(mock_session_persistence));

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

        let result = use_cases.find_sessions_by_filters(filters).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert_eq!(msg, "Invalid start date"),
            _ => panic!("Expected BadRequest for invalid start date"),
        }
    }
}
