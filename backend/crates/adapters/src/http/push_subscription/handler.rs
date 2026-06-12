use crate::http::app_state::AppState;
use crate::http::model::session_model::UserSession;
use crate::http_error::{HttpError, HttpResult};
use application::shared::use_cases::push_subscription::save_push_subscription::SavePushSubscriptionError;
use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushSubscriptionKeysDto {
    pub p256dh: String,
    pub auth: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavePushSubscriptionDto {
    pub endpoint: String,
    pub keys: PushSubscriptionKeysDto,
}

#[derive(Debug, Serialize)]
pub struct SavePushSubscriptionResponseDto {
    pub id: String,
}

impl From<SavePushSubscriptionError> for HttpError {
    fn from(e: SavePushSubscriptionError) -> Self {
        HttpError::GenericError(e.to_string())
    }
}

pub async fn save_push_subscription_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
    Json(payload): Json<SavePushSubscriptionDto>,
) -> HttpResult<Json<SavePushSubscriptionResponseDto>> {
    let id = state
        .save_push_subscription_uc
        .execute(
            user.user_id,
            payload.endpoint,
            payload.keys.p256dh,
            payload.keys.auth,
        )
        .await?;

    Ok(Json(SavePushSubscriptionResponseDto { id: id.to_string() }))
}
