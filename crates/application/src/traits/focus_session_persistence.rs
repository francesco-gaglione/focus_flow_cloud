use crate::app_error::AppResult;
use crate::use_cases::persistance_command::create_focus_session_data::CreateSessionData;
use crate::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData;
use crate::use_cases::persistance_command::find_session_by_filters_data::FindSessionByFiltersData;
use crate::use_cases::persistance_command::update_focus_session_data::UpdateFocusSessionData;
use async_trait::async_trait;
use domain::entities::focus_session::FocusSession;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FocusSessionPersistence: Send + Sync {
    async fn find_by_filters(
        &self,
        filters: FindSessionByFiltersData,
    ) -> AppResult<Vec<FocusSession>>;

    async fn create_manual_session(
        &self,
        session: &CreateManualSessionData,
    ) -> AppResult<FocusSession>;

    async fn create_session(&self, session: CreateSessionData) -> AppResult<FocusSession>;

    async fn update_session(&self, session: UpdateFocusSessionData) -> AppResult<()>;

    async fn find_session_by_id(&self, session_id: Uuid) -> AppResult<FocusSession>;
}
