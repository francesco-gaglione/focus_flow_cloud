use crate::{
    persistence::schema,
    persistence::{
        db_models::db_user::{DbUser, NewDbUser, UpdateDbUser},
        PostgresPersistence,
    },
};
use application::repository_traits::persistence_error::{PersistenceError, PersistenceResult};
use application::repository_traits::user_persistence::UserPersistence;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::user::entities::user::User;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

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

    #[instrument(skip(self))]
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
                    error!("User not found: user_id={user_id}");
                    PersistenceError::NotFound("User not found".to_string())
                }
                _ => PersistenceError::Unexpected(e.to_string()),
            })?;

        Ok(result.into())
    }

    #[instrument(skip(self))]
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
                    warn!("User not found");
                    PersistenceError::NotFound("User not found".to_string())
                }
                _ => PersistenceError::Unexpected(e.to_string()),
            })?;

        info!("User found in database");

        Ok(result.into())
    }

    #[instrument(skip(self))]
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

    #[instrument(skip(self))]
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

    #[instrument(skip(self))]
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

        Ok(role == "admin")
    }
}

#[cfg(test)]
mod tests {
    use crate::persistence::persistence_impl::persistence::postgres_persistence;
    use application::repository_traits::persistence_error::PersistenceError;
    use application::repository_traits::user_persistence::UserPersistence;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use domain::user::entities::user::User;
    use domain::user::entities::user_role::UserRole;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;
    use uuid::Uuid;

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations/");

    async fn setup() -> (
        crate::persistence::PostgresPersistence,
        testcontainers::ContainerAsync<Postgres>,
    ) {
        let container = Postgres::default().start().await.unwrap();
        let host = container.get_host().await.unwrap();
        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let db_url = format!("postgres://postgres:postgres@{host}:{port}/postgres");

        let persistence = postgres_persistence(&db_url).await;

        let conn = persistence.pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()).unwrap())
            .await
            .unwrap();

        (persistence, container)
    }

    fn make_user(role: UserRole) -> User {
        User::new(
            format!("user_{}", Uuid::new_v4()),
            "hashed_password".to_string(),
            role,
        )
    }

    #[tokio::test]
    async fn test_create_user_returns_id() {
        let (persistence, _container) = setup().await;
        let user = make_user(UserRole::User);

        let id = persistence.create_user(user).await.unwrap();
        assert_ne!(id, Uuid::nil());
    }

    #[tokio::test]
    async fn test_find_user_by_id() {
        let (persistence, _container) = setup().await;
        let user = make_user(UserRole::User);
        let username = user.username().to_string();

        let id = persistence.create_user(user).await.unwrap();
        let found = persistence.find_user_by_id(id).await.unwrap();

        assert_eq!(found.id(), id);
        assert_eq!(found.username(), username);
    }

    #[tokio::test]
    async fn test_find_user_by_id_not_found() {
        let (persistence, _container) = setup().await;

        let err = persistence
            .find_user_by_id(Uuid::new_v4())
            .await
            .unwrap_err();
        assert!(matches!(err, PersistenceError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_find_user_by_username() {
        let (persistence, _container) = setup().await;
        let user = make_user(UserRole::User);
        let username = user.username().to_string();

        persistence.create_user(user).await.unwrap();
        let found = persistence.find_user_by_username(&username).await.unwrap();

        assert_eq!(found.username(), username);
    }

    #[tokio::test]
    async fn test_find_user_by_username_not_found() {
        let (persistence, _container) = setup().await;

        let err = persistence
            .find_user_by_username("nonexistent_user")
            .await
            .unwrap_err();
        assert!(matches!(err, PersistenceError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_update_user_username_and_password() {
        let (persistence, _container) = setup().await;
        let user = make_user(UserRole::User);
        let id = persistence.create_user(user.clone()).await.unwrap();

        let mut updated = User::reconstitute(
            id,
            "updated_username".to_string(),
            "new_hashed_password".to_string(),
            UserRole::User,
        );
        updated.update_username("updated_username".to_string());
        updated.update_password("new_hashed_password".to_string());

        persistence.update_user(updated).await.unwrap();

        let found = persistence.find_user_by_id(id).await.unwrap();
        assert_eq!(found.username(), "updated_username");
        assert_eq!(found.hashed_password(), "new_hashed_password");
    }

    #[tokio::test]
    async fn test_delete_user() {
        let (persistence, _container) = setup().await;
        let user = make_user(UserRole::User);
        let id = persistence.create_user(user).await.unwrap();

        persistence.delete_user(id).await.unwrap();

        let err = persistence.find_user_by_id(id).await.unwrap_err();
        assert!(matches!(err, PersistenceError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_delete_user_not_found() {
        let (persistence, _container) = setup().await;

        let err = persistence.delete_user(Uuid::new_v4()).await.unwrap_err();
        assert!(matches!(err, PersistenceError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_is_user_admin_returns_true_for_admin() {
        let (persistence, _container) = setup().await;
        let user = make_user(UserRole::Admin);
        let id = persistence.create_user(user).await.unwrap();

        assert!(persistence.is_user_admin(id).await.unwrap());
    }

    #[tokio::test]
    async fn test_is_user_admin_returns_false_for_regular_user() {
        let (persistence, _container) = setup().await;
        let user = make_user(UserRole::User);
        let id = persistence.create_user(user).await.unwrap();

        assert!(!persistence.is_user_admin(id).await.unwrap());
    }

    #[tokio::test]
    async fn test_is_user_admin_not_found() {
        let (persistence, _container) = setup().await;

        let err = persistence.is_user_admin(Uuid::new_v4()).await.unwrap_err();
        assert!(matches!(err, PersistenceError::NotFound(_)));
    }
}
