use async_trait::async_trait;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::adapters::{
    persistence::{
        db_models::db_user_setting::{DbUserSetting, NewDbUserSetting, UpdateDbUserSetting},
        PostgresPersistence,
    },
    schema,
};
use application::{
    app_error::{AppError, AppResult},
    traits::user_setting_persistence::UserSettingPersistence,
};
use domain::entities::user_setting::UserSetting;

#[async_trait]
impl UserSettingPersistence for PostgresPersistence {
    async fn find_all(&self) -> AppResult<Vec<UserSetting>> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::user_settings::table
                    .select(DbUserSetting::as_select())
                    .order(schema::user_settings::created_at.desc())
                    .load(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .map_err(|e| AppError::Database(e.to_string()))?;

        let tasks: Vec<UserSetting> = result.into_iter().map(|c| c.into()).collect();
        Ok(tasks)
    }

    async fn update_setting(&self, key: String, value: String) -> AppResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let result = conn
            .interact(move |conn| {
                diesel::update(schema::user_settings::table)
                    .filter(schema::user_settings::key.eq(key.clone()))
                    .set(&UpdateDbUserSetting { key, value })
                    .returning(DbUserSetting::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .map_err(|e| AppError::Database(e.to_string()))?;

        match result {
            None => Err(AppError::GenericError(
                "UserSetting not updated".to_string(),
            )),
            Some(_) => Ok(()),
        }
    }

    async fn create_setting(&self, key: String, value: String) -> AppResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let _ = conn
            .interact(move |conn| {
                diesel::insert_into(schema::user_settings::table)
                    .values(&NewDbUserSetting { key, value })
                    .returning(DbUserSetting::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
