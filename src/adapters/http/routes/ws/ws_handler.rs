use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::{IntoResponse, Response},
};
use futures_util::{SinkExt, StreamExt};
use serde_json;
use tokio::sync::Mutex;
use tracing::{debug, error, warn};
use validator::Validate;

use crate::adapters::http::{
    app_state::{AppState, Clients},
    dto::ws_msg::{
        sync_workspace_ws::SyncWorkspace,
        ws_message::{WsErrorResponse, WsMessage, WsRequest, WsResponse, WsSuccessResponse},
    },
    routes::ws::{
        handle_break_event::handle_break_event, handle_start_event::handle_start_event,
        handle_terminate_event::handle_terminate_event,
        handle_update_concentration_score::handle_update_concentration_score,
        note_update::note_update, sync_workspace::sync_workspace,
        update_workspace::update_workspace,
    },
};

pub async fn session_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
        .into_response()
}

async fn handle_socket(ws: WebSocket, state: AppState) {
    static NEXT_ID: Mutex<usize> = Mutex::const_new(0);

    let my_id = {
        let mut id_lock = NEXT_ID.lock().await;
        let id = *id_lock;
        *id_lock += 1;
        id
    };

    debug!("Client {} connected", my_id);

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
                    debug!("Client {} received: {}", my_id, text);

                    match serde_json::from_str::<WsRequest>(&text) {
                        Ok(ws_request) => {
                            let request_id = ws_request.request_id.clone();

                            if let Err(validation_errors) = validate_message(&ws_request.message) {
                                error!(
                                    "Validation failed for client {}: {}",
                                    my_id, validation_errors
                                );

                                send_error_to_client(
                                    &tx_clone,
                                    "VALIDATION_ERROR",
                                    validation_errors.as_ref(),
                                    request_id,
                                )
                                .await;
                                continue;
                            }

                            match &ws_request.message {
                                WsMessage::RequestSync => {
                                    match sync_workspace(&state_for_receive).await {
                                        Ok(msg) => {
                                            send_sync_to_client(&tx_clone, msg).await;
                                        }
                                        Err(e) => {
                                            error!("Failed to sync clients: {:?}", e);
                                            send_error_to_client(
                                                &tx_clone,
                                                "ERROR",
                                                e.as_ref(),
                                                request_id,
                                            )
                                            .await;
                                        }
                                    }
                                }
                                WsMessage::UpdateWorkspace(update_workspace_msg) => {
                                    match update_workspace(update_workspace_msg, &state_for_receive)
                                        .await
                                    {
                                        Ok(msg) => {
                                            debug!("Workspace updated: {:?}", msg);

                                            send_success_to_client(
                                                &tx_clone,
                                                "Workspace updated",
                                                request_id.clone(),
                                            )
                                            .await;

                                            broadcast_message(
                                                &clients_clone,
                                                my_id,
                                                &WsMessage::UpdateWorkspace(msg),
                                                true,
                                            )
                                            .await;
                                        }
                                        Err(e) => {
                                            error!("Failed to update workspace: {:?}", e);
                                            send_error_to_client(
                                                &tx_clone,
                                                "ERROR",
                                                e.as_ref(),
                                                request_id,
                                            )
                                            .await;
                                        }
                                    }
                                }
                                WsMessage::StartEvent => {
                                    match handle_start_event(&state_for_receive).await {
                                        Ok(msg) => {
                                            debug!("Session started: {:?}", msg);

                                            send_success_to_client(
                                                &tx_clone,
                                                "Session started",
                                                request_id.clone(),
                                            )
                                            .await;

                                            broadcast_message(
                                                &clients_clone,
                                                my_id,
                                                &WsMessage::StartSession(msg),
                                                true,
                                            )
                                            .await;
                                        }
                                        Err(e) => {
                                            error!("Failed to start session: {:?}", e);
                                            send_error_to_client(
                                                &tx_clone,
                                                "ERROR",
                                                e.as_ref(),
                                                request_id,
                                            )
                                            .await;
                                        }
                                    }
                                }
                                WsMessage::BreakEvent => {
                                    match handle_break_event(&state_for_receive).await {
                                        Ok(msg) => {
                                            debug!("Break session started: {:?}", msg);

                                            send_success_to_client(
                                                &tx_clone,
                                                "Break session started",
                                                request_id.clone(),
                                            )
                                            .await;

                                            broadcast_message(
                                                &clients_clone,
                                                my_id,
                                                &WsMessage::StartSession(msg),
                                                true,
                                            )
                                            .await;
                                        }
                                        Err(e) => {
                                            error!("Failed to start break session: {:?}", e);
                                            send_error_to_client(
                                                &tx_clone,
                                                "ERROR",
                                                e.as_ref(),
                                                request_id,
                                            )
                                            .await;
                                        }
                                    }
                                }
                                WsMessage::TerminateEvent => {
                                    match handle_terminate_event(&state_for_receive).await {
                                        Ok(_) => {
                                            debug!("Session terminated");

                                            send_success_to_client(
                                                &tx_clone,
                                                "Session terminated",
                                                request_id.clone(),
                                            )
                                            .await;

                                            broadcast_message(
                                                &clients_clone,
                                                my_id,
                                                &WsMessage::TerminateSession,
                                                true,
                                            )
                                            .await;
                                        }
                                        Err(e) => {
                                            error!("Failed to terminate session: {:?}", e);
                                            send_error_to_client(
                                                &tx_clone,
                                                "ERROR",
                                                e.as_ref(),
                                                request_id,
                                            )
                                            .await;
                                        }
                                    }
                                }
                                WsMessage::UpdateConcentrationScore(concentration_score_dto) => {
                                    match handle_update_concentration_score(
                                        concentration_score_dto,
                                        &state_for_receive,
                                    )
                                    .await
                                    {
                                        Ok(_) => {
                                            debug!("Concentration score updated");

                                            send_success_to_client(
                                                &tx_clone,
                                                "Concentration score updated",
                                                request_id.clone(),
                                            )
                                            .await;

                                            broadcast_message(
                                                &clients_clone,
                                                my_id,
                                                &WsMessage::UpdateConcentrationScore(
                                                    concentration_score_dto.clone(),
                                                ),
                                                true,
                                            )
                                            .await;
                                        }
                                        Err(e) => {
                                            error!("Failed to update concentration score: {}", e);
                                            send_error_to_client(
                                                &tx_clone,
                                                "ERROR",
                                                e.as_ref(),
                                                request_id,
                                            )
                                            .await;
                                        }
                                    }
                                }
                                WsMessage::NoteUpdate(note_update_dto) => {
                                    debug!("Note update: {:?}", note_update_dto);

                                    match note_update(note_update_dto, &state_for_receive).await {
                                        Ok(_) => {
                                            debug!("Note updated");

                                            send_success_to_client(
                                                &tx_clone,
                                                "Note updated",
                                                request_id.clone(),
                                            )
                                            .await;

                                            broadcast_message(
                                                &clients_clone,
                                                my_id,
                                                &WsMessage::NoteUpdate(note_update_dto.clone()),
                                                true,
                                            )
                                            .await;
                                        }
                                        Err(e) => {
                                            error!("Failed to update note: {}", e);
                                            send_error_to_client(
                                                &tx_clone,
                                                "ERROR",
                                                e.as_ref(),
                                                request_id,
                                            )
                                            .await;
                                        }
                                    }
                                }
                                _ => {
                                    debug!("Server received a not handled message");
                                    send_error_to_client(
                                        &tx_clone,
                                        "ERROR",
                                        "Server is not able to handle this message",
                                        request_id,
                                    )
                                    .await;
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse JSON from client {}: {}", my_id, e);

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
                    debug!("Client {} requested close", my_id);
                    break;
                }
                Ok(Message::Ping(_)) => {
                    debug!("Received ping from client {}", my_id);
                }
                Ok(Message::Pong(_)) => {
                    debug!("Received pong from client {}", my_id);
                }
                Err(e) => {
                    error!("WebSocket error for client {}: {}", my_id, e);
                    break;
                }
                _ => {
                    warn!("Received unexpected message type from client {}", my_id);
                }
            }
        }
        debug!("Receive task ending for client {}", my_id);
    });

    tokio::select! {
        _ = send_task => debug!("Send task completed for client {}", my_id),
        _ = receive_task => debug!("Receive task completed for client {}", my_id),
    }

    state.ws_clients.write().await.remove(&my_id);
    debug!("Client {} disconnected", my_id);
}

async fn send_success_to_client(
    tx: &tokio::sync::mpsc::UnboundedSender<Message>,
    message: &str,
    request_id: Option<String>,
) {
    let response = WsResponse::Success(WsSuccessResponse {
        message: message.to_string(),
        request_id,
    });

    if let Ok(json) = serde_json::to_string(&response) {
        let _ = tx.send(Message::text(json));
    }
}

async fn send_sync_to_client(
    tx: &tokio::sync::mpsc::UnboundedSender<Message>,
    message: SyncWorkspace,
) {
    let response = WsResponse::Sync(message);

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
    let response = WsResponse::Error(WsErrorResponse {
        code: code.to_string(),
        message: message.to_string(),
        request_id,
    });
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = tx.send(Message::text(json));
    }
}

macro_rules! validate_variant {
    ($msg:expr, $variant:literal) => {
        $msg.validate()
            .map_err(|e| format!("{} validation failed: {}", $variant, e))
    };
}

fn validate_message(message: &WsMessage) -> Result<(), String> {
    match message {
        WsMessage::RequestSync
        | WsMessage::StartEvent
        | WsMessage::BreakEvent
        | WsMessage::TerminateEvent
        | WsMessage::TerminateSession => Ok(()),

        WsMessage::NoteUpdate(msg) => validate_variant!(msg, "NoteUpdate"),
        WsMessage::UpdateWorkspace(msg) => validate_variant!(msg, "UpdateWorkspace"),
        WsMessage::UpdateConcentrationScore(msg) => {
            validate_variant!(msg, "UpdateConcentrationScore")
        }
        WsMessage::StartSession(msg) => validate_variant!(msg, "StartSession"),
    }
}

async fn broadcast_message(
    clients: &Clients,
    sender_id: usize,
    message: &WsMessage,
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
