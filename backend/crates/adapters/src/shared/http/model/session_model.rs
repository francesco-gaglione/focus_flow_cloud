use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const SESSION_KEY: &str = "user_session";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub user_id: Uuid,
}
