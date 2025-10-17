use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::adapters::http::dto::ws_msg::{
    note_update_ws::NoteUpdate, start_session_ws::StartSession, sync_workspace_ws::SyncWorkspace,
    update_concentration_score::UpdateConcentrationScore, update_workspace_ws::UpdateWorkspace,
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
    // From client message
    RequestSync,

    StartEvent,
    BreakEvent,
    TerminateEvent,

    // Both to and from client
    NoteUpdate(NoteUpdate),
    UpdateConcentrationScore(UpdateConcentrationScore),
    UpdateWorkspace(UpdateWorkspace),

    // To client message
    StartSession(StartSession),
    TerminateSession,
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
