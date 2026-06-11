use crate::flashcards::persistence::db_models::db_flashcard::{
    DbFlashcard, DbFlashcardFolderItem, NewDbFlashcard, UpdateDbFlashcard,
};
use crate::shared::persistence::schema;
use crate::shared::persistence::PostgresPersistence;
use application::flashcards::traits::flashcard_persistence::FlashcardPersistence;
use application::shared::traits::persistence_error::{PersistenceError, PersistenceResult};
use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::flashcards::entities::flashcard::Flashcard;
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
}
