use std::collections::HashMap;
use std::sync::Arc;

use application::repository_traits::pomodoro_state_repository::{
    PomodoroStateRepository, PomodoroStateRepositoryError, PomodoroStateResult,
};
use async_trait::async_trait;
use domain::entities::focus_session::{FocusSession, RunningSession, TerminatedSession};
use domain::entities::pomodoro::pomodoro_state::PomodoroState;
use tokio::sync::RwLock;
use tracing::debug;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct PomodoroStateInMermoryImpl {
    stores: Arc<RwLock<HashMap<Uuid, PomodoroStateInMemoryStore>>>,
}

#[derive(Clone, Debug)]
pub struct PomodoroStateInMemoryStore {
    pub user_id: Uuid,
    pub selected_task_id: Option<Uuid>,
    pub running_session: Option<FocusSession<RunningSession>>,
    pub consecutive: Vec<FocusSession<TerminatedSession>>,
}

impl PomodoroStateInMemoryStore {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            selected_task_id: None,
            running_session: None,
            consecutive: Vec::new(),
        }
    }
}

impl From<PomodoroStateInMemoryStore> for PomodoroState {
    fn from(value: PomodoroStateInMemoryStore) -> Self {
        let mut state = Self::new();

        if let Some(task_id) = value.selected_task_id {
            state.update_task_id(task_id);
        }
        if let Some(session) = value.running_session {
            state.restore_running_session(value.user_id, session);
        }
        for session in value.consecutive {
            state.add_consecutive_session(session.clone());
        }

        state
    }
}

impl From<PomodoroState> for PomodoroStateInMemoryStore {
    fn from(mut value: PomodoroState) -> Self {
        Self {
            user_id: value.user_id(),
            selected_task_id: value.task_id(),
            running_session: value.current_session(),
            consecutive: value.consecutive_sessions().to_vec(),
        }
    }
}

impl PomodoroStateInMermoryImpl {
    pub fn new() -> Self {
        Self {
            stores: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for PomodoroStateInMermoryImpl {
    fn default() -> Self {
        Self {
            stores: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl PomodoroStateRepository for PomodoroStateInMermoryImpl {
    async fn init_user_state(&self, user_id: Uuid) -> PomodoroStateResult<()> {
        let mut stores = self.stores.write().await;
        if stores.contains_key(&user_id) {
            return Ok(());
        }
        stores.insert(user_id, PomodoroStateInMemoryStore::new(user_id));
        Ok(())
    }

    async fn fetch_user_state(&self, user_id: Uuid) -> PomodoroStateResult<PomodoroState> {
        debug!("Fetching user state for user: {:?}", user_id);
        let stores = self.stores.read().await;
        let user_state = stores
            .get(&user_id)
            .ok_or(PomodoroStateRepositoryError::UserNotFound)?;
        debug!(
            "Fetched user state for user {:?}, state: {:?}",
            user_id, user_state
        );

        Ok(user_state.clone().into())
    }

    async fn update_user_state(
        &self,
        user_id: Uuid,
        state: PomodoroState,
    ) -> PomodoroStateResult<()> {
        let mut stores = self.stores.write().await;
        let user_state = stores
            .get_mut(&user_id)
            .ok_or(PomodoroStateRepositoryError::UserNotFound)?;
        *user_state = state.into();
        Ok(())
    }

    async fn clear_user_state(&self, user_id: Uuid) -> PomodoroStateResult<()> {
        let mut stores = self.stores.write().await;
        stores.remove(&user_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use application::repository_traits::pomodoro_state_repository::{
        PomodoroStateRepository, PomodoroStateRepositoryError,
    };

    fn make_repo() -> PomodoroStateInMermoryImpl {
        PomodoroStateInMermoryImpl::new()
    }

    #[tokio::test]
    async fn test_init_creates_state() {
        let repo = make_repo();
        let user_id = Uuid::new_v4();

        repo.init_user_state(user_id).await.unwrap();
        let state = repo.fetch_user_state(user_id).await.unwrap();
        assert_eq!(state.user_id(), Uuid::nil());
    }

    #[tokio::test]
    async fn test_init_is_idempotent() {
        let repo = make_repo();
        let user_id = Uuid::new_v4();

        repo.init_user_state(user_id).await.unwrap();
        repo.init_user_state(user_id).await.unwrap();

        // Should still be exactly one entry
        let state = repo.fetch_user_state(user_id).await.unwrap();
        assert_eq!(state.user_id(), Uuid::nil());
    }

    #[tokio::test]
    async fn test_fetch_unknown_user_returns_error() {
        let repo = make_repo();
        let err = repo.fetch_user_state(Uuid::new_v4()).await.unwrap_err();
        assert!(matches!(err, PomodoroStateRepositoryError::UserNotFound));
    }

    #[tokio::test]
    async fn test_update_unknown_user_returns_error() {
        let repo = make_repo();
        let state = PomodoroState::new();
        let err = repo
            .update_user_state(Uuid::new_v4(), state)
            .await
            .unwrap_err();
        assert!(matches!(err, PomodoroStateRepositoryError::UserNotFound));
    }

    #[tokio::test]
    async fn test_clear_removes_state() {
        let repo = make_repo();
        let user_id = Uuid::new_v4();

        repo.init_user_state(user_id).await.unwrap();
        repo.clear_user_state(user_id).await.unwrap();

        let err = repo.fetch_user_state(user_id).await.unwrap_err();
        assert!(matches!(err, PomodoroStateRepositoryError::UserNotFound));
    }

    #[tokio::test]
    async fn test_clear_nonexistent_user_is_ok() {
        let repo = make_repo();
        let result = repo.clear_user_state(Uuid::new_v4()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_default_creates_empty_repo() {
        let repo = PomodoroStateInMermoryImpl::default();
        let err = repo.fetch_user_state(Uuid::new_v4()).await.unwrap_err();
        assert!(matches!(err, PomodoroStateRepositoryError::UserNotFound));
    }
}
