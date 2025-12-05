use crate::adapters::schema;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use domain::{entities::category::Category, error::persistence_error::PersistenceError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::categories)]
pub struct NewDbCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::categories)]
pub struct UpdateDbCategory {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

impl From<Category> for NewDbCategory {
    fn from(value: Category) -> Self {
        Self {
            id: value.id(),
            name: value.name().to_string(),
            description: value.description().map(|s| s.to_string()),
            color: value.color().to_string(),
        }
    }
}

impl From<Category> for UpdateDbCategory {
    fn from(value: Category) -> Self {
        Self {
            name: Some(value.name().to_string()),
            description: value.description().map(|s| s.to_string()),
            color: Some(value.color().to_string()),
        }
    }
}

impl TryFrom<DbCategory> for Category {
    type Error = PersistenceError;

    fn try_from(value: DbCategory) -> Result<Self, Self::Error> {
        Self::reconstitute(value.id, value.name, value.description, value.color)
            .map_err(|e| PersistenceError::Unexpected(format!("{}", e)))
    }
}
