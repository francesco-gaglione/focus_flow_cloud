use async_trait::async_trait;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::persistence::{
    db_models::db_user_setting::{DbUserSetting, NewDbUserSetting, UpdateDbUserSetting},
    schema, PostgresPersistence,
};
use domain::entities::user_setting::UserSetting;
use domain::error::persistence_error::{PersistenceError, PersistenceResult};
use domain::traits::user_setting_persistence::UserSettingPersistence;

#[async_trait]
impl UserSettingPersistence for PostgresPersistence {
    async fn find_all(&self) -> PersistenceResult<Vec<UserSetting>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::user_settings::table
                    .select(DbUserSetting::as_select())
                    .order(schema::user_settings::created_at.desc())
                    .load(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let tasks: Vec<UserSetting> = result.into_iter().map(|c| c.into()).collect();
        Ok(tasks)
    }

    async fn update_setting(
        &self,
        user_id: uuid::Uuid,
        key: String,
        value: String,
    ) -> PersistenceResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;
        let result = conn
            .interact(move |conn| {
                diesel::update(schema::user_settings::table)
                    .filter(schema::user_settings::key.eq(key.clone()))
                    .filter(schema::user_settings::user_id.eq(user_id))
                    .set(&UpdateDbUserSetting { key, value })
                    .returning(DbUserSetting::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match result {
            None => Err(PersistenceError::Unexpected(
                "UserSetting not updated".to_string(),
            )),
            Some(_) => Ok(()),
        }
    }

    async fn create_setting(
        &self,
        user_id: uuid::Uuid,
        key: String,
        value: String,
    ) -> PersistenceResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let _ = conn
            .interact(move |conn| {
                diesel::insert_into(schema::user_settings::table)
                    .values(&NewDbUserSetting {
                        user_id,
                        key,
                        value,
                    })
                    .returning(DbUserSetting::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        Ok(())
    }
}
