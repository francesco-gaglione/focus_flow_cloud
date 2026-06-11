use crate::shared::http::app_state::AppState;
use crate::shared::http::model::session_model::UserSession;
use crate::http_error::{map_persistence_error, HttpError, HttpResult};
use application::shared::use_cases::reminder::get_pending_reminders::GetPendingRemindersError;
use axum::extract::State;
use axum::Extension;
use axum::Json;
use serde::{Deserialize, Serialize};

impl From<GetPendingRemindersError> for HttpError {
    fn from(value: GetPendingRemindersError) -> Self {
        match value {
            GetPendingRemindersError::PersistenceError(e) => map_persistence_error(e),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PendingReminderDto {
    pub id: String,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub date_time: i64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPendingRemindersResponseDto {
    pub reminders: Vec<PendingReminderDto>,
}

pub async fn get_pending_reminders_api(
    State(state): State<AppState>,
    Extension(user): Extension<UserSession>,
) -> HttpResult<Json<GetPendingRemindersResponseDto>> {
    let reminders = state.shared.get_pending_reminders_uc.execute(user.user_id).await?;

    Ok(Json(GetPendingRemindersResponseDto {
        reminders: reminders
            .into_iter()
            .map(|r| PendingReminderDto {
                id: r.id.to_string(),
                date_time: r.date_time.timestamp(),
                title: r.title,
                description: r.description,
            })
            .collect(),
    }))
}
