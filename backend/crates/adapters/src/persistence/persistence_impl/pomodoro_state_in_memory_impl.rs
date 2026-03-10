use std::collections::HashMap;
use std::sync::Arc;

use application::repository_traits::pomodoro_state_repository::{
    PomodoroStateRepository, PomodoroStateRepositoryError, PomodoroStateResult,
};
use async_trait::async_trait;
use domain::entities::focus_session::{FocusSession, RunningSession, TerminatedSession};
use domain::entities::pomodoro::pomodoro_state::PomodoroState;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct PomodoroStateInMermoryImpl {
    stores: Arc<RwLock<HashMap<Uuid, PomodoroStateInMemoryStore>>>,
}

#[derive(Clone, Debug)]
pub struct PomodoroStateInMemoryStore {
    pub user_id: Uuid,
    pub selected_category_id: Option<Uuid>,
    pub selected_task_id: Option<Uuid>,
    pub running_session: Option<FocusSession<RunningSession>>,
    pub consecutive: Vec<FocusSession<TerminatedSession>>,
}

impl From<PomodoroStateInMemoryStore> for PomodoroState {
    fn from(value: PomodoroStateInMemoryStore) -> Self {
        let mut state = Self::new();

        if let Some(category_id) = value.selected_category_id {
            state.update_category_id(category_id);
        };
        if let Some(task_id) = value.selected_task_id {
            state.update_task_id(task_id);
        }
        if let Some(session) = value.running_session {
            state
                .start_new_session(value.user_id, session.session_type(), None, None)
                .unwrap();
            let current_session: &mut FocusSession<RunningSession> =
                state.current_session().as_mut().unwrap();
            if let Some(task_id) = session.task_id() {
                current_session.update_task_id(task_id);
            }
            if let Some(category_id) = session.category_id() {
                current_session.update_category_id(category_id);
            }
            if let Some(notes) = session.note() {
                current_session.update_note(notes);
            }
        }
        for session in value.consecutive {
            state.add_consecutive_session(session.clone());
        }

        state
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
    async fn fetch_user_state(&self, user_id: Uuid) -> PomodoroStateResult<PomodoroState> {
        let stores = self.stores.read().await;
        let user_state = stores
            .get(&user_id)
            .ok_or(PomodoroStateRepositoryError::UserNotFound)?;

        Ok(user_state.clone().into())
    }

    async fn update_work_context(
        &self,
        user_id: Uuid,
        category_id: Option<Uuid>,
        task_id: Option<Uuid>,
    ) -> PomodoroStateResult<()> {
        let mut stores = self.stores.write().await;
        let user_state = stores
            .get_mut(&user_id)
            .ok_or(PomodoroStateRepositoryError::UserNotFound)?;

        user_state.selected_category_id = category_id;
        user_state.selected_task_id = task_id;

        Ok(())
    }

    async fn store_running_session(
        &self,
        user_id: Uuid,
        session: FocusSession<RunningSession>,
    ) -> PomodoroStateResult<()> {
        let mut stores = self.stores.write().await;
        let user_state = stores
            .get_mut(&user_id)
            .ok_or(PomodoroStateRepositoryError::UserNotFound)?;

        if user_state.running_session.is_some() {
            return Err(PomodoroStateRepositoryError::AlreadyRunning);
        }

        user_state.running_session = Some(session);

        Ok(())
    }

    async fn update_running_session(
        &self,
        user_id: Uuid,
        session: FocusSession<RunningSession>,
    ) -> PomodoroStateResult<()> {
        let mut stores = self.stores.write().await;
        let user_state = stores
            .get_mut(&user_id)
            .ok_or(PomodoroStateRepositoryError::UserNotFound)?;

        if user_state.running_session.is_none() {
            return Err(PomodoroStateRepositoryError::SessionNotFound);
        }

        user_state.running_session = Some(session);

        Ok(())
    }

    async fn clear_running_session(&self, user_id: Uuid) -> PomodoroStateResult<()> {
        let mut stores = self.stores.write().await;
        let user_state = stores
            .get_mut(&user_id)
            .ok_or(PomodoroStateRepositoryError::UserNotFound)?;

        user_state.running_session = None;

        Ok(())
    }

    async fn fetch_running_session(
        &self,
        user_id: Uuid,
    ) -> PomodoroStateResult<FocusSession<RunningSession>> {
        let stores = self.stores.read().await;
        let user_state = stores
            .get(&user_id)
            .ok_or(PomodoroStateRepositoryError::UserNotFound)?;

        user_state
            .running_session
            .clone()
            .ok_or(PomodoroStateRepositoryError::SessionNotFound)
    }
}
