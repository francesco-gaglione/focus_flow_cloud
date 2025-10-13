use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::adapters::http::dto::ws_msg::{
    note_update_ws::NoteUpdate, start_session_ws::StartSession,
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
    StartSession(StartSession),
    NoteUpdate(NoteUpdate),
}

// Responses

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum WsResponse {
    Success(WsSuccessResponse),
    Error(WsErrorResponse),
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
    // pub code: String, //TODO define error codes
    pub message: String,
    pub request_id: Option<String>,
}
