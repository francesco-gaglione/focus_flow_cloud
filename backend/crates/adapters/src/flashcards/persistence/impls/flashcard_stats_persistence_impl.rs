use crate::shared::persistence::schema;
use crate::shared::persistence::PostgresPersistence;
use application::flashcards::traits::flashcard_stats_persistence::{
    ActivityEntry, FlashcardStatsPersistence, FolderFlashcardStats, GlobalFlashcardStats,
};
use application::shared::traits::persistence_error::{PersistenceError, PersistenceResult};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
impl FlashcardStatsPersistence for PostgresPersistence {
    #[instrument(skip(self))]
    async fn get_global_stats(&self, user_id: &Uuid) -> PersistenceResult<GlobalFlashcardStats> {
        let user_id = *user_id;
        let now = Utc::now();
        let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        conn.interact(move |conn| {
            let total_cards: i64 = schema::flashcards::table
                .filter(schema::flashcards::user_id.eq(user_id))
                .count()
                .get_result(conn)?;

            let due_today: i64 = schema::flashcards::table
                .filter(schema::flashcards::user_id.eq(user_id))
                .filter(schema::flashcards::due_date.le(Some(now)))
                .count()
                .get_result(conn)?;

            let reviewed_today: i64 = schema::flashcard_reviews::table
                .filter(schema::flashcard_reviews::user_id.eq(user_id))
                .filter(schema::flashcard_reviews::reviewed_at.ge(today_start))
                .count()
                .get_result(conn)?;

            let thirty_days_ago = now - chrono::Duration::days(30);

            let total_30d: i64 = schema::flashcard_reviews::table
                .filter(schema::flashcard_reviews::user_id.eq(user_id))
                .filter(schema::flashcard_reviews::reviewed_at.ge(thirty_days_ago))
                .count()
                .get_result(conn)?;

            let retained_30d: i64 = schema::flashcard_reviews::table
                .filter(schema::flashcard_reviews::user_id.eq(user_id))
                .filter(schema::flashcard_reviews::reviewed_at.ge(thirty_days_ago))
                .filter(schema::flashcard_reviews::rating.ne("Again"))
                .count()
                .get_result(conn)?;

            let retention_rate_30d = if total_30d == 0 {
                0.0
            } else {
                retained_30d as f64 / total_30d as f64
            };

            Ok::<GlobalFlashcardStats, diesel::result::Error>(GlobalFlashcardStats {
                total_cards,
                due_today,
                reviewed_today,
                retention_rate_30d,
            })
        })
        .await
        .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
        .map_err(|e| PersistenceError::Unexpected(e.to_string()))
    }

    #[instrument(skip(self))]
    async fn get_folder_stats(
        &self,
        folder_id: &Uuid,
        user_id: &Uuid,
    ) -> PersistenceResult<FolderFlashcardStats> {
        let folder_id = *folder_id;
        let user_id = *user_id;
        let now = Utc::now();
        let today_start: DateTime<Utc> = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        conn.interact(move |conn| {
            let total_cards: i64 = schema::flashcards::table
                .inner_join(schema::flashcard_folder_items::table)
                .filter(schema::flashcard_folder_items::folder_id.eq(folder_id))
                .count()
                .get_result(conn)?;

            let due_today: i64 = schema::flashcards::table
                .inner_join(schema::flashcard_folder_items::table)
                .filter(schema::flashcard_folder_items::folder_id.eq(folder_id))
                .filter(schema::flashcards::due_date.le(Some(now)))
                .count()
                .get_result(conn)?;

            // Use raw SQL for the 3-table join (reviews → folder_items by flashcard_id)
            #[derive(diesel::QueryableByName)]
            struct CountRow {
                #[diesel(sql_type = diesel::sql_types::BigInt)]
                cnt: i64,
            }

            let reviewed_row: CountRow = diesel::sql_query(
                "SELECT COUNT(*) AS cnt \
                 FROM flashcard_reviews fr \
                 INNER JOIN flashcard_folder_items ffi ON fr.flashcard_id = ffi.flashcard_id \
                 WHERE ffi.folder_id = $1 \
                 AND fr.user_id = $2 \
                 AND fr.reviewed_at >= $3",
            )
            .bind::<diesel::sql_types::Uuid, _>(folder_id)
            .bind::<diesel::sql_types::Uuid, _>(user_id)
            .bind::<diesel::sql_types::Timestamptz, _>(today_start)
            .get_result(conn)?;

            Ok::<FolderFlashcardStats, diesel::result::Error>(FolderFlashcardStats {
                total_cards,
                due_today,
                reviewed_today: reviewed_row.cnt,
            })
        })
        .await
        .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
        .map_err(|e| PersistenceError::Unexpected(e.to_string()))
    }

    #[instrument(skip(self))]
    async fn get_activity_heatmap(
        &self,
        user_id: &Uuid,
        days: u32,
    ) -> PersistenceResult<Vec<ActivityEntry>> {
        let user_id = *user_id;
        let now = Utc::now();
        let start = now - chrono::Duration::days(days as i64);

        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        // Raw SQL query to group by date
        let rows: Vec<(NaiveDate, i64)> = conn
            .interact(move |conn| {
                use diesel::sql_query;
                use diesel::sql_types::{BigInt, Date};

                #[derive(diesel::QueryableByName)]
                struct Row {
                    #[diesel(sql_type = Date)]
                    day: NaiveDate,
                    #[diesel(sql_type = BigInt)]
                    cnt: i64,
                }

                let results: Vec<Row> = sql_query(
                    "SELECT DATE(reviewed_at AT TIME ZONE 'UTC') AS day, COUNT(*) AS cnt \
                     FROM flashcard_reviews \
                     WHERE user_id = $1 AND reviewed_at >= $2 \
                     GROUP BY day \
                     ORDER BY day",
                )
                .bind::<diesel::sql_types::Uuid, _>(user_id)
                .bind::<diesel::sql_types::Timestamptz, _>(start)
                .load(conn)?;

                Ok::<Vec<(NaiveDate, i64)>, diesel::result::Error>(
                    results.into_iter().map(|r| (r.day, r.cnt)).collect(),
                )
            })
            .await
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?
            .map_err(|e| PersistenceError::Unexpected(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|(date, count)| ActivityEntry { date, count })
            .collect())
    }
}
