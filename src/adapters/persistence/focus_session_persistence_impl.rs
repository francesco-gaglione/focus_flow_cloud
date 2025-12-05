use crate::adapters::persistence::db_models::db_focus_session::{
    DbFocusSession, NewDbFocusSession, UpdateDbFocusSession,
};
use crate::adapters::persistence::PostgresPersistence;
use crate::adapters::schema;
use crate::application::app_error::{AppError, AppResult};
use crate::application::traits::focus_session_persistence::FocusSessionPersistence;
use crate::application::use_cases::persistance_command::create_focus_session_data::CreateSessionData;
use crate::application::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData;
use crate::application::use_cases::persistance_command::find_session_by_filters_data::FindSessionByFiltersData;
use crate::application::use_cases::persistance_command::update_focus_session_data::UpdateFocusSessionData;
use crate::domain::entities::focus_session::FocusSession;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::{error, info};
use uuid::Uuid;

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
            .map_err(|e| {
                error!("Error creating manual session: {}", e);
                AppError::Database("Focus session not created".to_string())
            })??;

        info!("Created focus session with id: {}", result.id);

        Ok(result.into())
    }

    async fn create_session(&self, session: CreateSessionData) -> AppResult<FocusSession> {
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
            .map_err(|e| {
                error!("Error creating manual session: {}", e);
                AppError::Database("Focus session not created".to_string())
            })??;

        info!("Created focus session with id: {}", result.id);

        Ok(result.into())
    }

    async fn find_session_by_id(&self, session_id: Uuid) -> AppResult<FocusSession> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result: DbFocusSession = conn
            .interact(move |conn| {
                use schema::focus_session::dsl::*;
                focus_session.find(session_id).get_result(conn)
            })
            .await
            .map_err(|e| {
                error!("Error finding focus session by id: {}", e);
                AppError::Database(e.to_string())
            })??;

        Ok(result.into())
    }

    async fn find_by_filters(
        &self,
        filters: FindSessionByFiltersData,
    ) -> AppResult<Vec<FocusSession>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                use schema::focus_session::dsl::*;
                let mut query = focus_session.into_boxed();

                if let Some(start) = filters.start_date {
                    query = query.filter(started_at.ge(start));
                }

                if let Some(end) = filters.end_date {
                    query = query.filter(started_at.le(end));
                }

                if let Some(cat_ids) = filters.category_ids {
                    query = query.filter(category_id.eq_any(cat_ids));
                }

                if let Some(session_t) = filters.session_type {
                    query = query.filter(session_type.eq(session_t.to_string()));
                }

                if let Some(min_score) = filters.min_concentration_score {
                    query = query.filter(concentration_score.ge(min_score));
                }

                if let Some(max_score) = filters.max_concentration_score {
                    query = query.filter(concentration_score.le(max_score));
                }

                query.load::<DbFocusSession>(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        Ok(result.into_iter().map(|s| s.into()).collect())
    }

    async fn update_session(&self, data: UpdateFocusSessionData) -> AppResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let session_id = data.session_id;
        let changeset = UpdateDbFocusSession::from(data);

        let _ = conn
            .interact(move |conn| {
                use schema::focus_session::dsl::*;
                diesel::update(focus_session.filter(id.eq(session_id)))
                    .set(changeset)
                    .get_result::<DbFocusSession>(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))??;

        Ok(())
    }
}
