use chrono::{DateTime, Utc};
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::adapters::schema;
use domain::entities::user_setting::UserSetting;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::user_settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUserSetting {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::user_settings)]
pub struct NewDbUserSetting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::user_settings)]
pub struct UpdateDbUserSetting {
    pub key: String,
    pub value: String,
}

impl From<DbUserSetting> for UserSetting {
    fn from(value: DbUserSetting) -> Self {
        UserSetting::new(value.key, Some(value.value))
    }
}
