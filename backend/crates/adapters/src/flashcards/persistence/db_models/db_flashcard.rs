use crate::shared::persistence::schema;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use domain::flashcards::entities::flashcard::Flashcard;
use domain::flashcards::value_objects::memory_state::MemoryState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::flashcards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFlashcard {
    pub id: Uuid,
    pub user_id: Uuid,
    pub front: String,
    pub back: String,
    pub stability: f32,
    pub difficulty: f32,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::flashcards)]
pub struct NewDbFlashcard {
    pub id: Uuid,
    pub user_id: Uuid,
    pub front: String,
    pub back: String,
    pub stability: f32,
    pub difficulty: f32,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::flashcards)]
pub struct UpdateDbFlashcard {
    pub front: Option<String>,
    pub back: Option<String>,
    pub stability: f32,
    pub difficulty: f32,
    #[diesel(treat_none_as_null = true)]
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::flashcard_folder_items)]
pub struct DbFlashcardFolderItem {
    pub flashcard_id: Uuid,
    pub folder_id: Uuid,
}

impl From<&Flashcard> for NewDbFlashcard {
    fn from(f: &Flashcard) -> Self {
        Self {
            id: *f.id(),
            user_id: *f.user_id(),
            front: f.front().to_string(),
            back: f.back().to_string(),
            stability: f.memory_state().stability(),
            difficulty: f.memory_state().difficulty(),
            due_date: f.due_date(),
        }
    }
}

impl From<&Flashcard> for UpdateDbFlashcard {
    fn from(f: &Flashcard) -> Self {
        Self {
            front: Some(f.front().to_string()),
            back: Some(f.back().to_string()),
            stability: f.memory_state().stability(),
            difficulty: f.memory_state().difficulty(),
            due_date: f.due_date(),
        }
    }
}

impl From<DbFlashcard> for Flashcard {
    fn from(row: DbFlashcard) -> Self {
        Flashcard::reconstitute(
            row.id,
            row.user_id,
            row.front,
            row.back,
            MemoryState::new(row.stability, row.difficulty),
            row.due_date,
        )
    }
}
