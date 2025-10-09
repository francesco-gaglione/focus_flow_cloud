use crate::adapters::persistence::PostgresPersistence;
use crate::adapters::persistence::db_models::db_focus_session::{
    DbFocusSession, NewDbFocusSession,
};
use crate::adapters::schema;
use crate::application::app_error::{AppError, AppResult};
use crate::application::traits::FocusSessionPersistence;
use crate::application::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData;
use crate::domain::entities::focus_session::FocusSession;
use async_trait::async_trait;
use diesel::{RunQueryDsl, SelectableHelper};
use tracing::info;

#[async_trait]
impl FocusSessionPersistence for PostgresPersistence {
    async fn create_manual_session(
        &self,
        session: &CreateManualSessionData,
    ) -> AppResult<FocusSession> {
        let session = session.clone();
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                diesel::insert_into(schema::focus_session::table)
                    .values(NewDbFocusSession::from(session))
                    .returning(DbFocusSession::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| AppError::Database("Focus session not created".to_string()))??;

        info!("Created focus session with id: {}", result.id);

        Ok(result.into())
    }
}
