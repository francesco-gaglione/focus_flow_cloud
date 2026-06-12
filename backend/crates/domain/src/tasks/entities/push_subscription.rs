use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PushSubscription {
    id: Uuid,
    user_id: Uuid,
    endpoint: String,
    p256dh: String,
    auth: String,
}

impl PushSubscription {
    pub fn new(user_id: Uuid, endpoint: String, p256dh: String, auth: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            endpoint,
            p256dh,
            auth,
        }
    }

    pub fn reconstitute(
        id: Uuid,
        user_id: Uuid,
        endpoint: String,
        p256dh: String,
        auth: String,
    ) -> Self {
        Self {
            id,
            user_id,
            endpoint,
            p256dh,
            auth,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
    pub fn p256dh(&self) -> &str {
        &self.p256dh
    }
    pub fn auth(&self) -> &str {
        &self.auth
    }
}
