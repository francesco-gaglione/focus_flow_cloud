use crate::flashcards::persistence::db_models::db_flashcard::{
    DbFlashcard, DbFlashcardFolderItem, NewDbFlashcard, UpdateDbFlashcard,
};
use crate::flashcards::persistence::db_models::db_flashcard_review::NewDbFlashcardReview;
use crate::flashcards::persistence::db_models::db_folder::{DbFolder, NewDbFolder};
use crate::shared::persistence::schema;
use crate::shared::persistence::PostgresPersistence;
use application::flashcards::traits::flashcard_persistence::FlashcardPersistence;
use application::shared::traits::persistence_error::{PersistenceError, PersistenceResult};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::flashcards::entities::flashcard::Flashcard;
use domain::shared::entities::folder_metadata::FolderMetadata;
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
impl FlashcardPersistence for PostgresPersistence {
    #[instrument(skip(self))]
    async fn save(&self, flashcard: &Flashcard) -> PersistenceResult<()> {
        let new_row = NewDbFlashcard::from(flashcard);

        self.with_transaction(move |conn| {
            diesel::insert_into(schema::flashcards::table)
                .values(&new_row)
                .execute(conn)?;
            Ok(())
        })
        .await
    }

    #[instrument(skip(self))]
    async fn save_to_folder(
        &self,
        flashcard: &Flashcard,
        folder_id: &Uuid,
    ) -> PersistenceResult<()> {
        let new_row = NewDbFlashcard::from(flashcard);
        let link = DbFlashcardFolderItem {
            flashcard_id: *flashcard.id(),
            folder_id: *folder_id,
        };

        self.with_transaction(move |conn| {
            diesel::insert_into(schema::flashcards::table)
                .values(&new_row)
                .execute(conn)?;

            diesel::insert_into(schema::flashcard_folder_items::table)
                .values(&link)
                .execute(conn)?;

            Ok(())
        })
        .await
    }

    #[instrument(skip(self))]
    async fn remove_from_folder(
        &self,
        flashcard: &Flashcard,
        folder_id: &Uuid,
    ) -> PersistenceResult<()> {
        let flashcard_id = *flashcard.id();
        let folder_id = *folder_id;

        let affected = self
            .with_transaction(move |conn| {
                diesel::delete(schema::flashcard_folder_items::table)
                    .filter(schema::flashcard_folder_items::flashcard_id.eq(flashcard_id))
                    .filter(schema::flashcard_folder_items::folder_id.eq(folder_id))
                    .execute(conn)
            })
            .await?;

        match affected > 0 {
            true => Ok(()),
            false => Err(PersistenceError::NotFound(format!(
                "Flashcard {} not found in folder {}",
                flashcard_id, folder_id
            ))),
        }
    }

    #[instrument(skip(self))]
    async fn find_by_folder(&self, folder_id: &Uuid) -> PersistenceResult<Vec<Flashcard>> {
        let folder_id = *folder_id;

        let rows = self
            .with_transaction(move |conn| {
                schema::flashcards::table
                    .inner_join(schema::flashcard_folder_items::table)
                    .filter(schema::flashcard_folder_items::folder_id.eq(folder_id))
                    .select(DbFlashcard::as_select())
                    .load::<DbFlashcard>(conn)
            })
            .await?;

        Ok(rows.into_iter().map(Flashcard::from).collect())
    }

    #[instrument(skip(self))]
    async fn update(&self, flashcard: &Flashcard) -> PersistenceResult<()> {
        let id = *flashcard.id();
        let update_row = UpdateDbFlashcard::from(flashcard);

        let affected = self
            .with_transaction(move |conn| {
                diesel::update(schema::flashcards::table)
                    .filter(schema::flashcards::id.eq(id))
                    .set(&update_row)
                    .execute(conn)
            })
            .await?;

        match affected > 0 {
            true => Ok(()),
            false => Err(PersistenceError::NotFound(format!(
                "Flashcard with id {} not found",
                id
            ))),
        }
    }

    #[instrument(skip(self))]
    async fn find_by_id(&self, id: Uuid) -> PersistenceResult<Flashcard> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::flashcards::table
                    .filter(schema::flashcards::id.eq(id))
                    .select(DbFlashcard::as_select())
                    .first(conn)
                    .optional()
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match result {
            Some(row) => Ok(Flashcard::from(row)),
            None => Err(PersistenceError::NotFound(format!(
                "Flashcard with id {} not found",
                id
            ))),
        }
    }

    #[instrument(skip(self))]
    async fn find_subfolders_by_parent(
        &self,
        parent_id: &Uuid,
    ) -> PersistenceResult<Vec<FolderMetadata>> {
        let parent_id = *parent_id;

        let rows = self
            .with_transaction(move |conn| {
                schema::flashcard_folders::table
                    .filter(schema::flashcard_folders::parent_id.eq(parent_id))
                    .select(DbFolder::as_select())
                    .load::<DbFolder>(conn)
            })
            .await?;

        Ok(rows.into_iter().map(FolderMetadata::from).collect())
    }

    #[instrument(skip(self))]
    async fn find_root_folder_by_user(&self, user_id: &Uuid) -> PersistenceResult<FolderMetadata> {
        let user_id = *user_id;

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        let result = conn
            .interact(move |conn| {
                schema::flashcard_folders::table
                    .filter(schema::flashcard_folders::user_id.eq(user_id))
                    .filter(schema::flashcard_folders::parent_id.is_null())
                    .select(DbFolder::as_select())
                    .first(conn)
                    .optional()
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        match result {
            Some(row) => Ok(FolderMetadata::from(row)),
            None => Err(PersistenceError::NotFound(format!(
                "Root folder for user {} not found",
                user_id
            ))),
        }
    }

    #[instrument(skip(self))]
    async fn create_root_folder_for_user(
        &self,
        user_id: &Uuid,
    ) -> PersistenceResult<FolderMetadata> {
        let folder = FolderMetadata::new("root", *user_id);
        let row = NewDbFolder {
            id: folder.id(),
            user_id: folder.user_id(),
            name: folder.name().to_string(),
            parent_id: None,
            path: "/".to_string(),
        };

        self.with_transaction(move |conn| {
            diesel::insert_into(schema::flashcard_folders::table)
                .values(&row)
                .execute(conn)?;
            Ok(())
        })
        .await?;

        Ok(folder)
    }

    #[instrument(skip(self))]
    async fn delete(&self, id: Uuid) -> PersistenceResult<()> {
        let affected = self
            .with_transaction(move |conn| {
                diesel::delete(schema::flashcards::table)
                    .filter(schema::flashcards::id.eq(id))
                    .execute(conn)
            })
            .await?;

        match affected > 0 {
            true => Ok(()),
            false => Err(PersistenceError::NotFound(format!(
                "Flashcard with id {} not found",
                id
            ))),
        }
    }

    #[instrument(skip(self))]
    async fn find_due_by_user(&self, user_id: &Uuid) -> PersistenceResult<Vec<Flashcard>> {
        let user_id = *user_id;
        let now = Utc::now();

        let rows = self
            .with_transaction(move |conn| {
                schema::flashcards::table
                    .filter(schema::flashcards::user_id.eq(user_id))
                    .filter(schema::flashcards::due_date.le(Some(now)))
                    .select(DbFlashcard::as_select())
                    .load::<DbFlashcard>(conn)
            })
            .await?;

        Ok(rows.into_iter().map(Flashcard::from).collect())
    }

    #[instrument(skip(self))]
    async fn create_folder(
        &self,
        name: &str,
        user_id: &Uuid,
        parent_id: &Uuid,
    ) -> PersistenceResult<FolderMetadata> {
        let user_id_val = *user_id;
        let parent_id_val = *parent_id;
        let name_owned = name.to_string();
        let new_id = Uuid::new_v4();

        let folder = self
            .with_transaction(move |conn| {
                let parent: DbFolder = schema::flashcard_folders::table
                    .filter(schema::flashcard_folders::id.eq(parent_id_val))
                    .select(DbFolder::as_select())
                    .first(conn)?;

                let path = if parent.path == "/" {
                    format!("/{}", new_id)
                } else {
                    format!("{}/{}", parent.path, new_id)
                };

                let row = NewDbFolder {
                    id: new_id,
                    user_id: user_id_val,
                    name: name_owned.clone(),
                    parent_id: Some(parent_id_val),
                    path: path.clone(),
                };

                diesel::insert_into(schema::flashcard_folders::table)
                    .values(&row)
                    .execute(conn)?;

                Ok(FolderMetadata::reconstitute(
                    new_id,
                    user_id_val,
                    name_owned,
                    Some(parent_id_val),
                    path,
                ))
            })
            .await?;

        Ok(folder)
    }

    #[instrument(skip(self))]
    async fn delete_folder(&self, id: Uuid) -> PersistenceResult<()> {
        let affected = self
            .with_transaction(move |conn| {
                diesel::delete(schema::flashcard_folders::table)
                    .filter(schema::flashcard_folders::id.eq(id))
                    .execute(conn)
            })
            .await?;

        match affected > 0 {
            true => Ok(()),
            false => Err(PersistenceError::NotFound(format!(
                "Folder with id {} not found",
                id
            ))),
        }
    }

    #[instrument(skip(self))]
    async fn save_review(
        &self,
        flashcard_id: Uuid,
        user_id: Uuid,
        rating: &str,
        reviewed_at: DateTime<Utc>,
    ) -> PersistenceResult<()> {
        let row = NewDbFlashcardReview {
            id: Uuid::new_v4(),
            flashcard_id,
            user_id,
            rating: rating.to_string(),
            reviewed_at,
        };

        self.with_transaction(move |conn| {
            diesel::insert_into(schema::flashcard_reviews::table)
                .values(&row)
                .execute(conn)?;
            Ok(())
        })
        .await
    }

    #[instrument(skip(self))]
    async fn find_due_by_folder(&self, folder_id: &Uuid) -> PersistenceResult<Vec<Flashcard>> {
        let folder_id = *folder_id;
        let now = Utc::now();

        let rows = self
            .with_transaction(move |conn| {
                schema::flashcards::table
                    .inner_join(schema::flashcard_folder_items::table)
                    .filter(schema::flashcard_folder_items::folder_id.eq(folder_id))
                    .filter(schema::flashcards::due_date.le(Some(now)))
                    .select(DbFlashcard::as_select())
                    .load::<DbFlashcard>(conn)
            })
            .await?;

        Ok(rows.into_iter().map(Flashcard::from).collect())
    }
}
