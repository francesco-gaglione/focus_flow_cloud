use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::{
    entities::user::User,
    error::persistence_error::{PersistenceError, PersistenceResult},
    traits::user_persistence::UserPersistence,
};
use uuid::Uuid;

use crate::{
    persistence::schema,
    persistence::{
        db_models::db_user::{DbUser, NewDbUser, UpdateDbUser},
        PostgresPersistence,
    },
};

#[async_trait]
impl UserPersistence for PostgresPersistence {
    async fn create_user(&self, user: User) -> PersistenceResult<Uuid> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let user = conn
            .interact(move |conn| {
                diesel::insert_into(schema::users::table)
                    .values(&NewDbUser {
                        username: user.username().to_string(),
                        hashed_password: user.hashed_password().to_string(),
                        role: user.role().to_string(),
                    })
                    .returning(DbUser::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        Ok(user.id)
    }

    async fn find_user_by_id(&self, user_id: Uuid) -> PersistenceResult<User> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::users::table
                    .find(user_id)
                    .select(DbUser::as_select())
                    .first(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    PersistenceError::NotFound("User not found".to_string())
                }
                _ => PersistenceError::Unexpected(e.to_string()),
            })?;

        Ok(result.into())
    }

    async fn find_user_by_username(&self, username: &str) -> PersistenceResult<User> {
        let username = username.to_string();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::users::table
                    .filter(schema::users::username.eq(username))
                    .select(DbUser::as_select())
                    .first(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    PersistenceError::NotFound("User not found".to_string())
                }
                _ => PersistenceError::Unexpected(e.to_string()),
            })?;

        Ok(result.into())
    }

    async fn update_user(&self, user: User) -> PersistenceResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let db_user: UpdateDbUser = user.clone().into();

        conn.interact(move |conn| {
            diesel::update(schema::users::table)
                .filter(schema::users::id.eq(user.id()))
                .set(&db_user)
                .execute(conn)
        })
        .await
        .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                PersistenceError::NotFound("User not found for update".to_string())
            }
            _ => PersistenceError::Unexpected(e.to_string()),
        })?;

        Ok(())
    }

    async fn delete_user(&self, user_id: Uuid) -> PersistenceResult<()> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let count = conn
            .interact(move |conn| {
                diesel::delete(schema::users::table)
                    .filter(schema::users::id.eq(user_id))
                    .execute(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        if count == 0 {
            return Err(PersistenceError::NotFound(
                "User not found for deletion".to_string(),
            ));
        }

        Ok(())
    }

    async fn is_user_admin(&self, user_id: Uuid) -> PersistenceResult<bool> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let role = conn
            .interact(move |conn| {
                schema::users::table
                    .find(user_id)
                    .select(schema::users::role)
                    .first::<String>(conn)
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    PersistenceError::NotFound("User not found".to_string())
                }
                _ => PersistenceError::Unexpected(e.to_string()),
            })?;

        Ok(role == "ADMIN")
    }
}
