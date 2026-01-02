use crate::http::model::session_model::UserSession;
use crate::http::request_id::RequestId;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, State,
    },
    response::{IntoResponse, Response},
};
use futures_util::{SinkExt, StreamExt};
use serde_json;
use tokio::sync::Mutex;
use tracing::{debug, error, warn, Instrument};
use uuid::Uuid;
use validator::Validate;

use crate::http::pomodoro_state::PomodoroState;
use crate::http::{
    app_state::{AppState, Clients},
    ws::{
        handle_break_event::handle_break_event,
        handle_start_event::handle_start_event,
        handle_terminate_event::handle_terminate_event,
        handle_update_concentration_score::handle_update_concentration_score,
        note_update::note_update,
        sync_pomodoro_state::sync_pomodoro_state,
        update_pomodoro_context::update_pomodoro_context,
        update_pomodoro_state::UpdatePomodoroState,
        ws_message::{BroadcastEvent, ClientMessage, ServerResponse, WsClientRequest},
    },
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn session_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Extension(user_session): Extension<UserSession>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, request_id, user_session.user_id))
        .into_response()
}

async fn handle_socket(ws: WebSocket, state: AppState, request_id: RequestId, user_id: Uuid) {
    static NEXT_ID: Mutex<usize> = Mutex::const_new(0);

    let my_id = {
        let mut id_lock = NEXT_ID.lock().await;
        let id = *id_lock;
        *id_lock += 1;
        id
    };

    let span = tracing::info_span!("websocket-connection", request_id = %request_id, client_id = my_id, user_id = %user_id);
    async move {
        debug!("Client connected");

        // Ensure user has a state
        {
            let mut states = state.pomodoro_states.write().await;
            states
                .entry(user_id)
                .or_insert_with(|| Arc::new(RwLock::new(PomodoroState::default())));
        }

        let (mut sender_ws, mut receiver_ws) = ws.split();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        state.ws_clients.write().await.insert(my_id, tx.clone());

        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if sender_ws.send(msg).await.is_err() {
                    debug!("Failed to send message to client, connection closed");
                    break;
                }
            }
        });

        let clients_clone = state.ws_clients.clone();
        let tx_clone = tx.clone();

        let state_for_receive = state.clone();

        let receive_task = tokio::spawn(async move {
            while let Some(result) = receiver_ws.next().await {
                match result {
                    Ok(Message::Text(text)) => {
                        debug!("Received: {}", text);

                        match serde_json::from_str::<WsClientRequest>(&text) {
                            Ok(ws_request) => {
                                let request_id = ws_request.request_id.clone();

                                if let Err(validation_errors) =
                                    validate_message(&ws_request.message)
                                {
                                    error!("Validation failed: {}", validation_errors);

                                    send_error_to_client(
                                        &tx_clone,
                                        "VALIDATION_ERROR",
                                        validation_errors.as_ref(),
                                        request_id,
                                    )
                                    .await;
                                    continue;
                                }

                                handle_message(
                                    my_id,
                                    ws_request.message,
                                    ws_request.request_id,
                                    &state_for_receive,
                                    &clients_clone,
                                    &tx_clone,
                                    user_id,
                                )
                                .await;
                            }
                            Err(e) => {
                                error!("Failed to parse JSON from client: {}", e);

                                send_error_to_client(
                                    &tx_clone,
                                    "PARSE_ERROR",
                                    "Failed to parse request",
                                    None,
                                )
                                .await;
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        debug!("Client requested close");
                        break;
                    }
                    Ok(Message::Ping(_)) => {
                        debug!("Received ping");
                    }
                    Ok(Message::Pong(_)) => {
                        debug!("Received pong");
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {
                        warn!("Received unexpected message type");
                    }
                }
            }
            debug!("Receive task ending");
        });

        tokio::select! {
            _ = send_task => debug!("Send task completed"),
            _ = receive_task => debug!("Receive task completed"),
        }

        state.ws_clients.write().await.remove(&my_id);
        debug!("Client disconnected");
    }
    .instrument(span)
    .await
}

async fn handle_message(
    my_id: usize,
    msg: ClientMessage,
    request_id: Option<String>,
    state: &AppState,
    clients: &Clients,
    tx: &tokio::sync::mpsc::UnboundedSender<Message>,
    user_id: Uuid,
) {
    match msg {
        ClientMessage::RequestSync => match sync_pomodoro_state(state, user_id).await {
            Ok(msg) => {
                send_sync_to_client(tx, msg).await;
            }
            Err(e) => {
                error!("Failed to sync clients: {:?}", e);
                send_error_to_client(tx, "ERROR", e.as_ref(), request_id).await;
            }
        },
        ClientMessage::UpdatePomodoroContext(context) => {
            match update_pomodoro_context(&context, state, user_id).await {
                Ok(msg) => {
                    debug!("Workspace updated: {:?}", msg);
                    send_success_to_client(tx, "Workspace updated", request_id.clone()).await;
                    broadcast_message(
                        clients,
                        my_id,
                        &BroadcastEvent::PomodoroSessionUpdate(msg),
                        true,
                    )
                    .await;
                }
                Err(e) => {
                    error!("Failed to update workspace: {:?}", e);
                    send_error_to_client(tx, "ERROR", e.as_ref(), request_id).await;
                }
            }
        }
        ClientMessage::StartEvent => match handle_start_event(state, user_id).await {
            Ok(msg) => {
                debug!("Session started: {:?}", msg);
                send_success_to_client(tx, "Session started", request_id.clone()).await;
                broadcast_message(
                    clients,
                    my_id,
                    &BroadcastEvent::PomodoroSessionUpdate(msg),
                    true,
                )
                .await;
            }
            Err(e) => {
                error!("Failed to start session: {:?}", e);
                send_error_to_client(tx, "ERROR", e.as_ref(), request_id).await;
            }
        },
        ClientMessage::BreakEvent => match handle_break_event(state, user_id).await {
            Ok(msg) => {
                debug!("Break session started: {:?}", msg);
                send_success_to_client(tx, "Break session started", request_id.clone()).await;
                broadcast_message(
                    clients,
                    my_id,
                    &BroadcastEvent::PomodoroSessionUpdate(msg),
                    true,
                )
                .await;
            }
            Err(e) => {
                error!("Failed to start break session: {:?}", e);
                send_error_to_client(tx, "ERROR", e.as_ref(), request_id).await;
            }
        },
        ClientMessage::TerminateEvent => match handle_terminate_event(state, user_id).await {
            Ok(msg) => {
                debug!("Session terminated");
                send_success_to_client(tx, "Session terminated", request_id.clone()).await;
                broadcast_message(
                    clients,
                    my_id,
                    &BroadcastEvent::PomodoroSessionUpdate(msg),
                    true,
                )
                .await;
            }
            Err(e) => {
                error!("Failed to terminate session: {:?}", e);
                send_error_to_client(tx, "ERROR", e.as_ref(), request_id).await;
            }
        },
        ClientMessage::UpdateNote(note_update_dto) => {
            match note_update(&note_update_dto, state, user_id).await {
                Ok(msg) => {
                    debug!("Note updated");
                    send_success_to_client(tx, "Note updated", request_id.clone()).await;
                    broadcast_message(
                        clients,
                        my_id,
                        &BroadcastEvent::PomodoroSessionUpdate(msg),
                        false,
                    )
                    .await;
                }
                Err(e) => {
                    error!("Failed to update note: {}", e);
                    send_error_to_client(tx, "ERROR", e.as_ref(), request_id).await;
                }
            }
        }
        ClientMessage::UpdateConcentrationScore(update_concentration_score) => {
            match handle_update_concentration_score(&update_concentration_score, state, user_id)
                .await
            {
                Ok(msg) => {
                    debug!("Concentration score updated");
                    send_success_to_client(tx, "Concentration score updated", request_id.clone())
                        .await;
                    broadcast_message(
                        clients,
                        my_id,
                        &BroadcastEvent::PomodoroSessionUpdate(msg),
                        true,
                    )
                    .await;
                }
                Err(e) => {
                    error!("Failed to update concentration score: {}", e);
                    send_error_to_client(tx, "ERROR", e.as_ref(), request_id).await;
                }
            }
        }
    }
}

// ============================================
// Validation Macro
// ============================================

macro_rules! validate_variant {
    ($msg:expr, $variant:literal) => {
        $msg.validate()
            .map_err(|e| format!("{} validation failed: {}", $variant, e))
    };
}

// ============================================
// Legacy Helper Functions (for backward compatibility)
// ============================================

async fn send_success_to_client(
    tx: &tokio::sync::mpsc::UnboundedSender<Message>,
    message: &str,
    request_id: Option<String>,
) {
    let response = ServerResponse::Success {
        message: message.to_string(),
        request_id,
    };

    if let Ok(json) = serde_json::to_string(&response) {
        let _ = tx.send(Message::text(json));
    }
}

async fn send_sync_to_client(
    tx: &tokio::sync::mpsc::UnboundedSender<Message>,

    message: UpdatePomodoroState,
) {
    let response = ServerResponse::SyncData(message);

    if let Ok(json) = serde_json::to_string(&response) {
        let _ = tx.send(Message::text(json));
    }
}

async fn send_error_to_client(
    tx: &tokio::sync::mpsc::UnboundedSender<Message>,
    code: &str,
    message: &str,
    request_id: Option<String>,
) {
    let response = ServerResponse::Error {
        code: code.to_string(),

        message: message.to_string(),

        request_id,
    };

    if let Ok(json) = serde_json::to_string(&response) {
        let _ = tx.send(Message::text(json));
    }
}

fn validate_message(message: &ClientMessage) -> Result<(), String> {
    match message {
        ClientMessage::RequestSync
        | ClientMessage::StartEvent
        | ClientMessage::BreakEvent
        | ClientMessage::TerminateEvent => Ok(()),

        ClientMessage::UpdateNote(msg) => validate_variant!(msg, "UpdateNote"),
        ClientMessage::UpdatePomodoroContext(msg) => {
            validate_variant!(msg, "UpdatePomodoroContext")
        }
        ClientMessage::UpdateConcentrationScore(msg) => {
            validate_variant!(msg, "UpdateConcentrationScore")
        }
    }
}

async fn broadcast_message(
    clients: &Clients,

    sender_id: usize,

    message: &BroadcastEvent,
    include_msg_sender: bool,
) {
    match serde_json::to_string(message) {
        Ok(json) => {
            let clients_read = clients.read().await;
            let mut sent_count = 0;

            for (&id, tx) in clients_read.iter() {
                if id != sender_id || include_msg_sender {
                    if tx.send(Message::text(json.clone())).is_ok() {
                        sent_count += 1;
                    } else {
                        warn!("Failed to send message to client {}", id);
                    }
                }
            }

            debug!(
                "Broadcasted message from client {} to {} clients",
                sender_id, sent_count
            );
        }
        Err(e) => {
            error!("Failed to serialize message: {}", e);
        }
    }
}
