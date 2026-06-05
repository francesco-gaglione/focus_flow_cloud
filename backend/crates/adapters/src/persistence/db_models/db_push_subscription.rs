use crate::persistence::schema;
use diesel::{Insertable, Queryable, Selectable};
use domain::tasks::entities::push_subscription::PushSubscription;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::push_subscriptions)]
pub struct DbPushSubscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::push_subscriptions)]
pub struct NewDbPushSubscription {
    pub user_id: Uuid,
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<DbPushSubscription> for PushSubscription {
    fn from(db: DbPushSubscription) -> Self {
        PushSubscription::reconstitute(db.id, db.user_id, db.endpoint, db.p256dh, db.auth)
    }
}
