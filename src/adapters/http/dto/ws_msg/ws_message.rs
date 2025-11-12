use serde::{Deserialize, Serialize};

use crate::adapters::http::dto::ws_msg::{
    note_update_ws::NoteUpdate, sync_workspace_ws::SyncWorkspace,
    update_concentration_score::UpdateConcentrationScore,
    update_pomodoro_state::UpdatePomodoroState,
};

// ============================================
// Client-to-Server Messages
// ============================================

/// Wrapper for all client requests with optional tracking ID
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRequest {
    pub request_id: Option<String>,
    #[serde(flatten)]
    pub message: ClientMessage,
}

/// Messages sent from client to server
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ClientMessage {
    RequestSync,
    StartEvent,
    BreakEvent,
    TerminateEvent,

    UpdateNote(NoteUpdate),
    UpdateConcentrationScore(UpdateConcentrationScore),
}

// ============================================
// Server-to-Client Responses
// ============================================

/// Direct responses from server to the requesting client
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServerResponse {
    Success {
        message: String,
        request_id: Option<String>,
    },
    Error {
        code: String,
        message: String,
        request_id: Option<String>,
    },
    SyncData(SyncWorkspace),
}

// ============================================
// Broadcast Events
// ============================================

/// Events broadcast to all connected clients (or all except sender)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum BroadcastEvent {
    PomodoroSessionUpdate(UpdatePomodoroState),
}
