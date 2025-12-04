use std::sync::Arc;

use crate::application::{
    app_error::AppResult,
    traits::focus_session_persistence::FocusSessionPersistence,
    use_cases::{
        focus_session::command::update_focus_session::UpdateFocusSessionCommand,
        persistance_command::update_focus_session_data::UpdateFocusSessionData,
    },
};

pub struct UpdateFocusSessionUseCase {
    session_persistence: Arc<dyn FocusSessionPersistence>,
}

impl UpdateFocusSessionUseCase {
    pub fn new(session_persistence: Arc<dyn FocusSessionPersistence>) -> Self {
        Self {
            session_persistence,
        }
    }

    pub async fn execute(&self, session: UpdateFocusSessionCommand) -> AppResult<()> {
        let session_data = UpdateFocusSessionData {
            session_id: session.session_id,
            task_id: session.task_id,
            category_id: session.category_id,
            concentration_score: session.concentration_score,
            notes: session.notes.clone(),
            started_at: session.started_at,
            ended_at: session.ended_at,
        };

        self.session_persistence.update_session(session_data).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::application::{
        traits::focus_session_persistence::MockFocusSessionPersistence,
        use_cases::focus_session::{
            command::update_focus_session::UpdateFocusSessionCommand,
            update_focus_session::UpdateFocusSessionUseCase,
        },
    };

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
