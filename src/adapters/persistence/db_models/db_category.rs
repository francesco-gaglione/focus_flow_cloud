use crate::application::use_cases::persistance_command::update_category_data::UpdateCategoryData;
use crate::domain::entities::category::Category;
use crate::{
    adapters::schema,
    application::use_cases::persistance_command::create_category_data::CreateCategoryData,
};
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
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::categories)]
pub struct NewDbCategory {
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

impl From<CreateCategoryData> for NewDbCategory {
    fn from(value: CreateCategoryData) -> Self {
        Self {
            name: value.name,
            description: value.description,
            color: value.color,
        }
    }
}

impl From<UpdateCategoryData> for UpdateDbCategory {
    fn from(value: UpdateCategoryData) -> Self {
        Self {
            name: value.name,
            description: value.description,
            color: value.color,
        }
    }
}

impl From<DbCategory> for Category {
    fn from(value: DbCategory) -> Self {
        Self::new(value.id, value.name, value.description, value.color)
    }
}
