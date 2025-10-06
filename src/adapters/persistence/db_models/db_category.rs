use crate::adapters::schema;
use crate::application::use_cases::commands::create_category::CreateCategoryCommand;
use crate::application::use_cases::commands::update_category::UpdateCategoryCommand;
use crate::domain::entities::category::Category;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::categories)]
pub struct NewDbCategory {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::categories)]
pub struct UpdateDbCategory {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

impl From<CreateCategoryCommand> for NewDbCategory {
    fn from(value: CreateCategoryCommand) -> Self {
        Self {
            name: value.name,
            description: value.description,
            color: value.color,
        }
    }
}

impl From<UpdateCategoryCommand> for UpdateDbCategory {
    fn from(value: UpdateCategoryCommand) -> Self {
        Self {
            name: value.name,
            description: value.description,
            color: value.color,
        }
    }
}

impl From<DbCategory> for Category {
    fn from(value: DbCategory) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            color: value.color,
        }
    }
}
