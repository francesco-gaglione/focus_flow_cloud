use crate::shared::traits::persistence_error::PersistenceError;
use crate::tasks::traits::focus_session_repository::FocusSessionRepository;
use chrono::{DateTime, Utc};
use domain::tasks::entities::focus_session::FocusSessionError;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq)]
pub enum UpdateFocusSessionError {
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("Focus session error: {0}")]
    FocusSessionError(#[from] FocusSessionError),
}

pub type UpdateFocusSessionResult<T> = Result<T, UpdateFocusSessionError>;

#[derive(Debug, Clone)]
pub struct UpdateFocusSessionCommand {
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

pub struct UpdateFocusSessionUseCase {
    session_persistence: Arc<dyn FocusSessionRepository>,
}

impl UpdateFocusSessionUseCase {
    pub fn new(session_persistence: Arc<dyn FocusSessionRepository>) -> Self {
        Self {
            session_persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        update_session: UpdateFocusSessionCommand,
    ) -> UpdateFocusSessionResult<()> {
        let mut session = self
            .session_persistence
            .find_session_by_id(update_session.session_id)
            .await?;

        if let Some(task_id) = update_session.task_id {
            session.update_task_id(task_id);
        }

        if let Some(concentration_score) = update_session.concentration_score {
            session.update_concentration_score(concentration_score)?;
        }

        if let Some(notes) = update_session.notes {
            session.update_note(notes);
        }

        if let (Some(start), Some(end)) = (update_session.started_at, update_session.ended_at) {
            session.update_date_range(start, end)?;
        }

        Ok(self.session_persistence.update_session(session).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::tasks::traits::focus_session_repository::MockFocusSessionRepository;
    use crate::tasks::use_cases::focus_session::update_focus_session::UpdateFocusSessionCommand;
    use crate::tasks::use_cases::focus_session::update_focus_session::UpdateFocusSessionUseCase;
    use domain::tasks::entities::focus_session::TerminatedSession;
    use domain::tasks::entities::{
        focus_session::FocusSession, focus_session_type::FocusSessionType,
    };

    #[tokio::test]
    async fn update_focus_session() {
        let session_id = uuid::Uuid::new_v4();
        let task_id = uuid::Uuid::new_v4();
        let concentration_score = 4;
        let notes = "Test notes".to_string();
        let started_at = chrono::Utc::now();
        let ended_at = chrono::Utc::now();

        let mut session_persistence = MockFocusSessionRepository::new();
        session_persistence
            .expect_update_session()
            .returning(|_| Ok(()));
        session_persistence
            .expect_find_session_by_id()
            .returning(move |_| {
                Ok(FocusSession::<TerminatedSession>::new(
                    uuid::Uuid::new_v4(),
                    Some(task_id.clone()),
                    FocusSessionType::Work,
                    Some(3600),
                    Some("Test notes".to_string()),
                    started_at,
                    ended_at,
                )
                .unwrap())
            });
        let use_case = UpdateFocusSessionUseCase::new(Arc::new(session_persistence));

        let result = use_case
            .execute(UpdateFocusSessionCommand {
                session_id,
                task_id: Some(task_id),
                concentration_score: Some(concentration_score),
                notes: Some(notes),
                started_at: Some(started_at),
                ended_at: Some(ended_at),
            })
            .await;

        assert!(result.is_ok());
    }
}
