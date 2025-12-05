use crate::app_error::AppResult;
use crate::use_cases::focus_session::command::create_foucs_session::CreateFocusSessionCommand;
use domain::entities::focus_session::FocusSession;
use domain::traits::focus_session_persistence::FocusSessionPersistence;
use std::sync::Arc;

pub struct CreateSessionUseCase {
    session_persistence: Arc<dyn FocusSessionPersistence>,
}

impl CreateSessionUseCase {
    pub fn new(session_persistence: Arc<dyn FocusSessionPersistence>) -> Self {
        Self {
            session_persistence,
        }
    }

    pub async fn execute(&self, session_cmd: CreateFocusSessionCommand) -> AppResult<FocusSession> {
        let session = FocusSession::new(
            session_cmd.category_id,
            session_cmd.task_id,
            session_cmd.session_type,
            session_cmd.concentration_score,
            session_cmd.notes,
            session_cmd.started_at,
            Some(session_cmd.ended_at),
        )?;

        Ok(self.session_persistence.create_session(session).await?)
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
    async fn test_create_session_success() {
        let mut mock_session_persistence = MockFocusSessionPersistence::new();

        let category_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let started_at = DateTime::from_timestamp(1761118663, 0).unwrap();
        let ended_at = DateTime::from_timestamp(1761118714, 0).unwrap();

        let focus_session = FocusSession::reconstitute(
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

        let use_case = CreateSessionUseCase::new(Arc::new(mock_session_persistence));
        let result = use_case.execute(cmd).await;

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
            .returning(|_| {
                Err(
                    domain::error::persistence_error::PersistenceError::Unexpected(
                        "Database error".to_string(),
                    ),
                )
            });

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

        let use_case = CreateSessionUseCase::new(Arc::new(mock_session_persistence));
        let result = use_case.execute(cmd).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Database(msg) => assert_eq!(msg, "Database error"),
            _ => panic!("Expected InternalServerError"),
        }
    }
}
