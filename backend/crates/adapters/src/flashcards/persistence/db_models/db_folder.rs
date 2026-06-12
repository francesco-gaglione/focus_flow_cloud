use crate::shared::persistence::schema;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use domain::shared::entities::folder_metadata::FolderMetadata;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::flashcard_folders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFolder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::flashcard_folders)]
pub struct NewDbFolder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub path: String,
}

impl From<DbFolder> for FolderMetadata {
    fn from(row: DbFolder) -> Self {
        FolderMetadata::reconstitute(row.id, row.user_id, row.name, row.parent_id, row.path)
    }
}
