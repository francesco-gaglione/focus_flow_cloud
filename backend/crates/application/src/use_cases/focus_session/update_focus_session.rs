use std::sync::Arc;

use crate::{
    app_error::AppResult,
    use_cases::focus_session::command::update_focus_session::UpdateFocusSessionCommand,
};
use domain::traits::focus_session_persistence::FocusSessionPersistence;

pub struct UpdateFocusSessionUseCase {
    session_persistence: Arc<dyn FocusSessionPersistence>,
}

impl UpdateFocusSessionUseCase {
    pub fn new(session_persistence: Arc<dyn FocusSessionPersistence>) -> Self {
        Self {
            session_persistence,
        }
    }

    pub async fn execute(&self, update_session: UpdateFocusSessionCommand) -> AppResult<()> {
        let mut session = self
            .session_persistence
            .find_session_by_id(update_session.session_id)
            .await?;

        if let Some(category_id) = update_session.category_id {
            session.update_category_id(Some(category_id));
        }

        if let Some(task_id) = update_session.task_id {
            session.update_task_id(Some(task_id));
        }

        if let Some(concentration_score) = update_session.concentration_score {
            session.update_concentration_score(Some(concentration_score));
        }

        if let Some(notes) = update_session.notes {
            session.update_notes(Some(notes));
        }

        if update_session.started_at.is_some() && update_session.ended_at.is_some() {
            session.update_date_range(
                update_session.started_at.unwrap(),
                Some(update_session.ended_at.unwrap()),
            )?;
        }

        Ok(self.session_persistence.update_session(session).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::mocks::MockFocusSessionPersistence;
    use crate::use_cases::focus_session::{
        command::update_focus_session::UpdateFocusSessionCommand,
        update_focus_session::UpdateFocusSessionUseCase,
    };
    use domain::entities::{focus_session::FocusSession, focus_session_type::FocusSessionType};

    #[tokio::test]
    async fn update_focus_session() {
        let session_id = uuid::Uuid::new_v4();
        let task_id = uuid::Uuid::new_v4();
        let category_id = uuid::Uuid::new_v4();
        let concentration_score = 80;
        let notes = "Test notes".to_string();
        let started_at = chrono::Utc::now();
        let ended_at = chrono::Utc::now();

        let mut session_persistence = MockFocusSessionPersistence::new();
        session_persistence
            .expect_update_session()
            .returning(|_| Ok(()));
        session_persistence
            .expect_find_session_by_id()
            .returning(move |_| {
                Ok(FocusSession::new(
                    uuid::Uuid::new_v4(),
                    Some(category_id.clone()),
                    Some(task_id.clone()),
                    FocusSessionType::Work,
                    Some(3600),
                    Some("Test notes".to_string()),
                    started_at,
                    Some(ended_at),
                )
                .unwrap())
            });
        let use_case = UpdateFocusSessionUseCase::new(Arc::new(session_persistence));

        let result = use_case
            .execute(UpdateFocusSessionCommand {
                session_id,
                task_id: Some(task_id),
                category_id: Some(category_id),
                concentration_score: Some(concentration_score),
                notes: Some(notes),
                started_at: Some(started_at),
                ended_at: Some(ended_at),
            })
            .await;

        assert!(result.is_ok());
    }
}
