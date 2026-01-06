use crate::persistence::db_models::db_focus_session::{
    DbFocusSession, NewDbFocusSession, UpdateDbFocusSession,
};
use crate::persistence::schema;
use crate::persistence::PostgresPersistence;
use async_trait::async_trait;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::entities::focus_session::{FocusSession, SessionFilter};
use domain::error::persistence_error::{PersistenceError, PersistenceResult};
use domain::traits::focus_session_persistence::FocusSessionPersistence;
use tracing::{error, info};
use uuid::Uuid;

#[async_trait]
impl FocusSessionPersistence for PostgresPersistence {
    async fn create_manual_session(
        &self,
        session: FocusSession,
    ) -> PersistenceResult<FocusSession> {
        let session = session.clone();
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

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
                PersistenceError::Unexpected("Focus session not created".to_string())
            })?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        info!("Created focus session with id: {}", result.id);

        Ok(result.into())
    }

    async fn create_session(&self, session: FocusSession) -> PersistenceResult<FocusSession> {
        let session = session.clone();
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

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
                PersistenceError::Unexpected("Focus session not created".to_string())
            })?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        info!("Created focus session with id: {}", result.id);

        Ok(result.into())
    }

    async fn find_session_by_id(&self, session_id: Uuid) -> PersistenceResult<FocusSession> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result: DbFocusSession = conn
            .interact(move |conn| {
                use schema::focus_session::dsl::*;
                focus_session.find(session_id).get_result(conn)
            })
            .await
            .map_err(|e| {
                error!("Error finding focus session by id: {}", e);
                PersistenceError::Unexpected(e.to_string())
            })?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        Ok(result.into())
    }

    async fn find_by_filters(
        &self,
        filters: SessionFilter,
    ) -> PersistenceResult<Vec<FocusSession>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                use schema::focus_session;
                use schema::tasks;

                let mut query = focus_session::table
                    .left_join(tasks::table)
                    .select(focus_session::all_columns)
                    .into_boxed();

                query = query.filter(focus_session::user_id.eq(filters.user_id));

                if let Some(start) = filters.start_date {
                    query = query.filter(focus_session::started_at.ge(start));
                }

                if let Some(end) = filters.end_date {
                    query = query.filter(focus_session::started_at.le(end));
                }

                if let Some(cat_ids) = filters.category_ids {
                    query = query.filter(
                        focus_session::category_id
                            .eq_any(cat_ids.clone())
                            .or(tasks::category_id.eq_any(cat_ids)),
                    );
                }

                if let Some(s_type) = filters.session_type {
                    query = query.filter(focus_session::session_type.eq(s_type.to_string()));
                }

                if let Some(min_score) = filters.min_concentration_score {
                    query = query.filter(focus_session::concentration_score.ge(min_score));
                }

                if let Some(max_score) = filters.max_concentration_score {
                    query = query.filter(focus_session::concentration_score.le(max_score));
                }

                if let Some(has_notes) = filters.has_notes {
                    if has_notes {
                        query = query.filter(focus_session::notes.is_not_null());
                    } else {
                        query = query.filter(focus_session::notes.is_null());
                    }
                }

                query.load::<DbFocusSession>(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result: Vec<DbFocusSession> = result; // Type hint
        info!("Found {} sessions", result.len());

        Ok(result.into_iter().map(|s| s.into()).collect())
    }

    async fn update_session(&self, session: FocusSession) -> PersistenceResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let session_id = session.id();
        let changeset = UpdateDbFocusSession::from(session);

        let _ = conn
            .interact(move |conn| {
                use schema::focus_session::dsl::*;
                diesel::update(focus_session.filter(id.eq(session_id)))
                    .set(changeset)
                    .get_result::<DbFocusSession>(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
