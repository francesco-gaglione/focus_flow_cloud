use crate::application::app_error::AppResult;
use crate::application::traits::FocusSessionPersistence;
use crate::application::use_cases::commands::create_manual_session::CreateManualFocusSessionCommand;
use crate::application::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData;
use crate::domain::entities::focus_session::FocusSession;
use std::sync::Arc;

pub struct FocusSessionUseCases {
    session_persistence: Arc<dyn FocusSessionPersistence>,
}

impl FocusSessionUseCases {
    pub fn new(session_persistence: Arc<dyn FocusSessionPersistence>) -> Self {
        Self {
            session_persistence,
        }
    }

    pub async fn create_session(
        &self,
        session: &CreateManualFocusSessionCommand,
    ) -> AppResult<FocusSession> {
        let duration_minutes = (session.ended_at.timestamp() - session.started_at.timestamp()) / 60;

        let manual_session_data = CreateManualSessionData {
            task_id: session.task_id,
            category_id: session.category_id,
            session_type: session.session_type.clone(),
            concentration_score: session.concentration_score,
            notes: session.notes.clone(),
            actual_duration_minutes: duration_minutes,
            started_at: session.started_at,
            ended_at: session.ended_at,
        };

        self.session_persistence
            .create_manual_session(&manual_session_data)
            .await
    }
}
