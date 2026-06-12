use crate::shared::traits::persistence_error::PersistenceResult;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use domain::tasks::entities::focus_session::FocusSession;
use domain::tasks::entities::focus_session::TerminatedSession;
use domain::tasks::entities::focus_session_type::FocusSessionType;
use uuid::Uuid;

pub struct FindByFiltersCommand {
    pub user_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub task_ids: Option<Vec<Uuid>>,
    pub session_type: Option<FocusSessionType>,
    pub min_concentration_score: Option<i32>,
    pub max_concentration_score: Option<i32>,
    pub has_notes: Option<bool>,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FocusSessionRepository: Send + Sync {
    async fn find_by_filters(
        &self,
        filters: FindByFiltersCommand,
    ) -> PersistenceResult<Vec<FocusSession<TerminatedSession>>>;

    async fn create_manual_session(
        &self,
        session: FocusSession<TerminatedSession>,
    ) -> PersistenceResult<()>;

    async fn update_session(
        &self,
        session: FocusSession<TerminatedSession>,
    ) -> PersistenceResult<()>;

    async fn find_session_by_id(
        &self,
        session_id: Uuid,
    ) -> PersistenceResult<FocusSession<TerminatedSession>>;
}
