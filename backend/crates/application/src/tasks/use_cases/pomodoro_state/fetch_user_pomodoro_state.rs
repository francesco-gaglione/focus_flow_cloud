use crate::tasks::traits::pomodoro_state_repository::{
    PomodoroStateRepository, PomodoroStateRepositoryError,
};
use domain::tasks::entities::focus_session::{FocusSession, RunningSession};
use domain::tasks::entities::focus_session_type::FocusSessionType;
use domain::tasks::entities::pomodoro::pomodoro_state::PomodoroState;
use std::sync::Arc;
use thiserror::Error;
use tracing::instrument;

#[derive(Debug, Error, PartialEq)]
pub enum FetchUserPomodoroStateError {
    #[error("Pomodoro repository error: {0}")]
    PomodoroRepositoryError(#[from] PomodoroStateRepositoryError),
}

pub type FetchUserPomodoroStateResult<T> = Result<T, FetchUserPomodoroStateError>;

pub struct FetchUserPomodoroStateOutput {
    pub task_id: Option<String>,
    pub user_current_session: Option<UserCurrentSession>,
}

pub struct UserCurrentSession {
    pub session_type: UserSessionType,
    pub session_start_time: i64,
    pub task_id: Option<String>,
    pub note: Option<String>,
    pub concentration_score: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserSessionType {
    Work,
    ShortBreak,
    LongBreak,
}

impl From<PomodoroState> for FetchUserPomodoroStateOutput {
    fn from(mut value: PomodoroState) -> Self {
        Self {
            task_id: value.task_id().map(|id| id.to_string()),
            user_current_session: value.current_session().as_ref().map(|s| s.into()),
        }
    }
}

impl From<&FocusSession<RunningSession>> for UserCurrentSession {
    fn from(value: &FocusSession<RunningSession>) -> Self {
        Self {
            session_type: value.session_type().into(),
            session_start_time: value.started_at().timestamp(),
            task_id: value.task_id().map(|id| id.to_string()),
            note: value.note(),
            concentration_score: 0,
        }
    }
}

impl From<FocusSessionType> for UserSessionType {
    fn from(value: FocusSessionType) -> Self {
        match value {
            FocusSessionType::Work => UserSessionType::Work,
            FocusSessionType::ShortBreak => UserSessionType::ShortBreak,
            FocusSessionType::LongBreak => UserSessionType::LongBreak,
        }
    }
}

#[derive(Debug)]
pub struct FetchUserPomodoroStateCommand {
    pub user_id: uuid::Uuid,
}

pub struct FetchUserPomodoroStateUseCase {
    pomodoro_state_repo: Arc<dyn PomodoroStateRepository>,
}

impl FetchUserPomodoroStateUseCase {
    pub fn new(pomodoro_state_repo: Arc<dyn PomodoroStateRepository>) -> Self {
        Self {
            pomodoro_state_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn execute(
        &self,
        command: FetchUserPomodoroStateCommand,
    ) -> FetchUserPomodoroStateResult<FetchUserPomodoroStateOutput> {
        let state = self
            .pomodoro_state_repo
            .fetch_user_state(command.user_id)
            .await?;

        Ok(state.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::traits::pomodoro_state_repository::{
        MockPomodoroStateRepository, PomodoroStateRepositoryError,
    };
    use domain::tasks::entities::pomodoro::pomodoro_state::PomodoroState;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_fetch_success_no_session() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        let state = PomodoroState::new();

        mock_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));

        let use_case = FetchUserPomodoroStateUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(FetchUserPomodoroStateCommand {
                user_id: Uuid::new_v4(),
            })
            .await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.user_current_session.is_none());
        assert!(output.task_id.is_none());
    }

    #[tokio::test]
    async fn test_fetch_success_with_running_session() {
        use domain::tasks::entities::focus_session_type::FocusSessionType;

        let mut mock_repo = MockPomodoroStateRepository::new();
        let user_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state
            .start_new_session(user_id, FocusSessionType::Work, None)
            .unwrap();

        mock_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));

        let use_case = FetchUserPomodoroStateUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(FetchUserPomodoroStateCommand { user_id })
            .await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.user_current_session.is_some());
        assert_eq!(
            output.user_current_session.unwrap().session_type,
            UserSessionType::Work
        );
    }

    #[tokio::test]
    async fn test_fetch_with_category_and_task() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        let task_id = Uuid::new_v4();
        let mut state = PomodoroState::new();
        state.update_task_id(task_id);

        mock_repo
            .expect_fetch_user_state()
            .returning(move |_| Ok(state.clone()));

        let use_case = FetchUserPomodoroStateUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(FetchUserPomodoroStateCommand {
                user_id: Uuid::new_v4(),
            })
            .await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.task_id, Some(task_id.to_string()));
    }

    #[tokio::test]
    async fn test_fetch_repository_error() {
        let mut mock_repo = MockPomodoroStateRepository::new();
        mock_repo
            .expect_fetch_user_state()
            .returning(|_| Err(PomodoroStateRepositoryError::UserNotFound));

        let use_case = FetchUserPomodoroStateUseCase::new(Arc::new(mock_repo));
        let result = use_case
            .execute(FetchUserPomodoroStateCommand {
                user_id: Uuid::new_v4(),
            })
            .await;
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            FetchUserPomodoroStateError::PomodoroRepositoryError(_)
        ));
    }
}
