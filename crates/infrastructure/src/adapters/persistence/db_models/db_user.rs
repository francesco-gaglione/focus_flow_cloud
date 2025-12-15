use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use domain::entities::{user::User, user_role::UserRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::adapters::schema;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUser {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct NewDbUser {
    pub username: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct UpdateDbUser {
    pub username: String,
    pub hashed_password: String,
}

impl From<DbUser> for User {
    fn from(value: DbUser) -> Self {
        Self::reconstitute(
            value.id,
            value.username,
            value.hashed_password,
            UserRole::from(value.role),
        )
    }
}

impl From<User> for UpdateDbUser {
    fn from(value: User) -> Self {
        Self {
            username: value.username().to_string(),
            hashed_password: value.hashed_password().to_string(),
        }
    }
}
