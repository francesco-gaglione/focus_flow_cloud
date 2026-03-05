use crate::persistence_traits::focus_session_persistence::FocusSessionPersistence;
use crate::persistence_traits::persistence_error::PersistenceError;
use chrono::{DateTime, Utc};
use domain::entities::{
    focus_session::{FocusSession, FocusSessionError},
    focus_session_type::FocusSessionType,
};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CreateManualSessionError {
    #[error("Invalid focus session")]
    InvalidFocusSession(#[from] FocusSessionError),

    #[error("Persistence error")]
    PersistenceError(#[from] PersistenceError),
}

pub type CreateManualSessionResult<T> = Result<T, CreateManualSessionError>;

#[derive(Debug, Clone)]
pub struct CreateManualFocusSessionCommand {
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub session_type: FocusSessionType,
    pub concentration_score: Option<i32>, // if none a default will be used (5)
    pub notes: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
}

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
    ) -> CreateManualSessionResult<FocusSession> {
        let session = FocusSession::new(
            session_cmd.user_id,
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
    use crate::persistence_traits::focus_session_persistence::MockFocusSessionPersistence;

    use super::*;
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
            Uuid::new_v4(), // user_id
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
            user_id: Uuid::new_v4(),
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
                    crate::persistence_traits::persistence_error::PersistenceError::Unexpected(
                        "Invalid session data".to_string(),
                    ),
                )
            });

        let cmd = CreateManualFocusSessionCommand {
            user_id: Uuid::new_v4(),
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
            CreateManualSessionError::PersistenceError(err) => {
                assert_eq!(
                    err,
                    PersistenceError::Unexpected("Invalid session data".to_string())
                );
            }
            _ => panic!("Expected Database error"),
        }
    }
}
