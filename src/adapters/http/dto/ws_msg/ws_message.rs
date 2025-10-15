use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::adapters::http::dto::ws_msg::{
    complete_session_ws::CompleteSession, note_update_ws::NoteUpdate,
    start_session_ws::StartSession, sync_workspace_ws::SyncWorkspace,
    update_workspace_ws::UpdateWorkspace,
};

// Requests

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WsRequest {
    pub request_id: Option<String>,
    #[serde(flatten)]
    pub message: WsMessage,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum WsMessage {
    RequestSync,
    StartSession(StartSession),
    CompleteSession(CompleteSession),
    EndSession,
    NoteUpdate(NoteUpdate),
    UpdateWorkspace(UpdateWorkspace),
}

// Responses

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum WsResponse {
    Success(WsSuccessResponse),
    Error(WsErrorResponse),
    Sync(SyncWorkspace),
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WsSuccessResponse {
    pub message: String,
    pub request_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WsErrorResponse {
    pub code: String, //TODO define error codes
    pub message: String,
    pub request_id: Option<String>,
}
