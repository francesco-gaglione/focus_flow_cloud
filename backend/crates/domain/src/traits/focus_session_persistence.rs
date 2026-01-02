use crate::entities::focus_session::FocusSession;
use crate::entities::focus_session::SessionFilter;
use crate::error::persistence_error::PersistenceResult;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FocusSessionPersistence: Send + Sync {
    async fn find_by_filters(&self, filters: SessionFilter)
        -> PersistenceResult<Vec<FocusSession>>;

    async fn create_manual_session(&self, session: FocusSession)
        -> PersistenceResult<FocusSession>;

    async fn create_session(&self, session: FocusSession) -> PersistenceResult<FocusSession>;

    async fn update_session(&self, session: FocusSession) -> PersistenceResult<()>;

    async fn find_session_by_id(&self, session_id: Uuid) -> PersistenceResult<FocusSession>;
}
