use crate::app_error::AppResult;
use crate::use_cases::focus_session::command::create_manual_session::CreateManualFocusSessionCommand;
use domain::entities::focus_session::FocusSession;
use domain::traits::focus_session_persistence::FocusSessionPersistence;
use std::sync::Arc;

#[derive(Clone)]
pub struct CreateManualSessionUseCase {
    session_persistence: Arc<dyn FocusSessionPersistence>,
}

impl CreateManualSessionUseCase {
    pub fn new(session_persistence: Arc<dyn FocusSessionPersistence>) -> Self {
        Self {
            session_persistence,
        }
    }

    pub async fn execute(
        &self,
        session_cmd: CreateManualFocusSessionCommand,
    ) -> AppResult<FocusSession> {
        let session = FocusSession::new(
            session_cmd.category_id,
            session_cmd.task_id,
            session_cmd.session_type,
            session_cmd.concentration_score,
            session_cmd.notes,
            session_cmd.started_at,
            Some(session_cmd.ended_at),
        )?;

        Ok(self
            .session_persistence
            .create_manual_session(session)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_error::AppError;
    use crate::mocks::MockFocusSessionPersistence;
    use chrono::DateTime;
    use domain::entities::focus_session_type::FocusSessionType;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_manual_session_success() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        let category_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let started_at = DateTime::from_timestamp(1761118663, 0).unwrap();
        let ended_at = DateTime::from_timestamp(1761118714, 0).unwrap();
        let duration = ended_at.timestamp() - started_at.timestamp();

        let focus_session = FocusSession::reconstitute(
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

        let cmd = CreateManualFocusSessionCommand {
            category_id: Some(category_id),
            task_id: Some(task_id),
            session_type: FocusSessionType::Work,
            concentration_score: Some(4),
            notes: Some("manual session notes".to_string()),
            started_at,
            ended_at,
        };

        let use_case = CreateManualSessionUseCase::new(Arc::new(mock_session_persistence));
        let result = use_case.execute(cmd).await;

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
            .returning(|_| {
                Err(
                    domain::error::persistence_error::PersistenceError::Unexpected(
                        "Invalid session data".to_string(),
                    ),
                )
            });

        let cmd = CreateManualFocusSessionCommand {
            category_id: Some(Uuid::new_v4()),
            task_id: Some(Uuid::new_v4()),
            session_type: FocusSessionType::Work,
            concentration_score: Some(3),
            notes: Some("notes".to_string()),
            started_at: DateTime::from_timestamp(1761118663, 0).unwrap(),
            ended_at: DateTime::from_timestamp(1761118714, 0).unwrap(),
        };

        let use_case = CreateManualSessionUseCase::new(Arc::new(mock_session_persistence));
        let result = use_case.execute(cmd).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Database(msg) => assert_eq!(msg, "Invalid session data"),
            _ => panic!("Expected Database error"),
        }
    }
}
