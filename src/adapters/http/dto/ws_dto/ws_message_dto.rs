use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::adapters::http::dto::ws_dto::{
    note_update::NoteUpdateDto, start_session_dto::StartSessionDto,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum WsMessage {
    StartSession(StartSessionDto),
    NoteUpdate(NoteUpdateDto),
}
