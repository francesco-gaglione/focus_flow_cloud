use crate::adapters::persistence::PostgresPersistence;
use crate::adapters::persistence::db_models::db_focus_session::{
    DbFocusSession, NewDbFocusSession,
};
use crate::adapters::schema;
use crate::application::app_error::{AppError, AppResult};
use crate::application::traits::FocusSessionPersistence;
use crate::application::use_cases::persistance_command::create_manual_session_data::CreateManualSessionData;
use crate::application::use_cases::persistance_command::find_session_by_filters_data::FindSessionByFiltersData;
use crate::domain::entities::focus_session::FocusSession;
use async_trait::async_trait;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use tracing::{error, info};

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
}
