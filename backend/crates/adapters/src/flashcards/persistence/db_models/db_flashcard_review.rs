use crate::shared::persistence::schema;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::flashcard_reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFlashcardReview {
    pub id: Uuid,
    pub flashcard_id: Uuid,
    pub user_id: Uuid,
    pub rating: String,
    pub reviewed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::flashcard_reviews)]
pub struct NewDbFlashcardReview {
    pub id: Uuid,
    pub flashcard_id: Uuid,
    pub user_id: Uuid,
    pub rating: String,
    pub reviewed_at: DateTime<Utc>,
}
