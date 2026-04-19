use std::sync::Arc;

use domain::entities::focus_session::FocusSessionError;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::repository_traits::{
    persistence_error::PersistenceError,
    pomodoro_state_repository::{PomodoroStateRepository, PomodoroStateRepositoryError},
};

#[derive(Debug, Error, PartialEq)]
pub enum UpdateSessionError {
    #[error("pomodoro state repository error: {0}")]
    PomodoroStateRepositoryError(#[from] PomodoroStateRepositoryError),

    #[error("focus session error: {0}")]
    FocusSessionError(#[from] FocusSessionError),

    #[error("persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),

    #[error("no current session found")]
    NoCurrentSession,
}

pub type UpdateSessionResult<T> = Result<T, UpdateSessionError>;

#[derive(Debug)]
pub struct UpdateSessionCommand {
    pub user_id: Uuid,
    pub new_note: Option<String>,
    pub new_concentration_score: Option<i32>,
}

pub struct UpdateSessionUseCase {
    pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
}

impl UpdateSessionUseCase {
    pub fn new(pomodoro_state_repo: Arc<dyn PomodoroStateRepository>) -> Self {
        Self {
            pomodoro_state_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, command: UpdateSessionCommand) -> UpdateSessionResult<()> {
        let mut user_state = self
            .pomodoro_state_repo
            .fetch_user_state(command.user_id)
            .await?;

        let mut current_session = user_state
            .current_session()
            .ok_or(UpdateSessionError::NoCurrentSession)?;

        if let Some(note) = command.new_note {
            current_session.update_note(note);
        }
        if let Some(concentration_score) = command.new_concentration_score {
            current_session.update_concentration_score(concentration_score)?;
        }

        user_state.update_current_session(current_session);

        self.pomodoro_state_repo
            .update_user_state(command.user_id, user_state)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_traits::pomodoro_state_repository::{
        MockPomodoroStateRepository, PomodoroStateRepositoryError,
    };
    use domain::entities::focus_session_type::FocusSessionType;
    use domain::entities::pomodoro::pomodoro_state::PomodoroState;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_update_note_success() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        let user_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state
            .start_new_session(user_id, FocusSessionType::Work, None, None)
            .unwrap();

        mock_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));
        mock_repo
            .expect_update_user_state()
            .returning(|_, _| Ok(()));

        let use_case = UpdateSessionUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(UpdateSessionCommand {
                user_id,
                new_note: Some("test note".to_string()),
                new_concentration_score: None,
            })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_concentration_score_success() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        let user_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state
            .start_new_session(user_id, FocusSessionType::Work, None, None)
            .unwrap();

        mock_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));
        mock_repo
            .expect_update_user_state()
            .returning(|_, _| Ok(()));

        let use_case = UpdateSessionUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(UpdateSessionCommand {
                user_id,
                new_note: None,
                new_concentration_score: Some(3),
            })
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_no_current_session() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        let user_id = Uuid::new_v4();
        let state = PomodoroState::new();

        mock_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));

        let use_case = UpdateSessionUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(UpdateSessionCommand {
                user_id,
                new_note: Some("note".to_string()),
                new_concentration_score: None,
            })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UpdateSessionError::NoCurrentSession
        ));
    }

    #[tokio::test]
    async fn test_update_fetch_repo_error() {
        let mut mock_repo = MockPomodoroStateRepository::new();

        mock_repo
            .expect_fetch_user_state()
            .returning(|_| Err(PomodoroStateRepositoryError::UserNotFound));

        let use_case = UpdateSessionUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(UpdateSessionCommand {
                user_id: Uuid::new_v4(),
                new_note: None,
                new_concentration_score: None,
            })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UpdateSessionError::PomodoroStateRepositoryError(_)
        ));
    }
}
