use chrono::DateTime;
use uuid::Uuid;

use crate::application::app_error::{AppError, AppResult};
use crate::application::traits::FocusSessionPersistence;
use crate::application::use_cases::commands::create_manual_session::CreateManualFocusSessionCommand;
use crate::application::use_cases::commands::find_session_filters::FindSessionFiltersCommand;
use crate::application::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData;
use crate::application::use_cases::persistance_command::find_session_by_filters_data::FindSessionByFiltersData;
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

    pub async fn find_sessions_by_filters(
        &self,
        filters: FindSessionFiltersCommand,
    ) -> AppResult<Vec<FocusSession>> {
        let start_date = match filters.date_range.as_ref() {
            Some(r) => Some(
                DateTime::from_timestamp_secs(r.start_date)
                    .ok_or_else(|| AppError::BadRequest("Invalid start date".to_string()))?
                    .into(),
            ),
            None => None,
        };

        let end_date = match filters.date_range.as_ref() {
            Some(r) => Some(
                DateTime::from_timestamp_secs(r.end_date)
                    .ok_or_else(|| AppError::BadRequest("Invalid end date".to_string()))?
                    .into(),
            ),
            None => None,
        };

        let category_ids = filters
            .category_ids
            .map(|ids| ids.iter().map(|id| Uuid::parse_str(id).unwrap()).collect());

        let min_concentration_score = filters.concentration_score_range.clone().map(|s| s.min);
        let max_concentration_score = filters.concentration_score_range.clone().map(|s| s.max);

        self.session_persistence
            .find_by_filters(FindSessionByFiltersData {
                start_date,
                end_date,
                category_ids,
                session_type: filters.session_type,
                min_concentration_score,
                max_concentration_score,
            })
            .await
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
