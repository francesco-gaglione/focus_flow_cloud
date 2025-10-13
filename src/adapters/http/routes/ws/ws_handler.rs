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
    dto::ws_msg::ws_message::{
        WsErrorResponse, WsMessage, WsRequest, WsResponse, WsSuccessResponse,
    },
    routes::ws::start_session::start_session,
};

pub async fn session_handler(ws: WebSocketUpgrade, state: State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
        .into_response()
}

async fn handle_socket(ws: WebSocket, State(state): State<AppState>) {
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
    let session_state_clone = state.focus_session_state.clone();
    let tx_clone = tx.clone();

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

                                send_error_to_client(&tx_clone, "VALIDATION_ERROR", request_id)
                                    .await;
                                continue;
                            }

                            match &ws_request.message {
                                WsMessage::StartSession(start_session_dto) => {
                                    match start_session(
                                        request_id.clone(),
                                        start_session_dto,
                                        &session_state_clone,
                                    )
                                    .await
                                    {
                                        Ok(response) => {
                                            debug!("Session started: {}", response);

                                            send_success_to_client(
                                                &tx_clone,
                                                &response,
                                                request_id.clone(),
                                            )
                                            .await;

                                            broadcast_message(
                                                &clients_clone,
                                                my_id,
                                                &ws_request.message,
                                            )
                                            .await;
                                        }
                                        Err(e) => {
                                            error!("Failed to start session: {:?}", e);
                                            send_error_to_client(
                                                &tx_clone,
                                                "SESSION_ERROR",
                                                request_id,
                                            )
                                            .await;
                                        }
                                    }
                                }
                                WsMessage::NoteUpdate(note_update_dto) => {
                                    debug!("Note update: {:?}", note_update_dto);

                                    send_success_to_client(
                                        &tx_clone,
                                        "Note updated successfully",
                                        request_id.clone(),
                                    )
                                    .await;

                                    broadcast_message(&clients_clone, my_id, &ws_request.message)
                                        .await;
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse JSON from client {}: {}", my_id, e);

                            send_error_to_client(&tx_clone, "PARSE_ERROR", None).await;
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

/// Invia un errore al client
async fn send_error_to_client(
    tx: &tokio::sync::mpsc::UnboundedSender<Message>,
    message: &str,
    request_id: Option<String>,
) {
    let response = WsResponse::Error(WsErrorResponse {
        message: message.to_string(),
        request_id,
    });

    if let Ok(json) = serde_json::to_string(&response) {
        let _ = tx.send(Message::text(json));
    }
}

fn validate_message(message: &WsMessage) -> Result<(), String> {
    match message {
        WsMessage::StartSession(dto) => dto
            .validate()
            .map_err(|e| format!("StartSession validation failed: {}", e)),
        WsMessage::NoteUpdate(dto) => dto
            .validate()
            .map_err(|e| format!("NoteUpdate validation failed: {}", e)),
    }
}

async fn broadcast_message(clients: &Clients, sender_id: usize, message: &WsMessage) {
    match serde_json::to_string(message) {
        Ok(json) => {
            let clients_read = clients.read().await;
            let mut sent_count = 0;

            for (&id, tx) in clients_read.iter() {
                if id != sender_id {
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
