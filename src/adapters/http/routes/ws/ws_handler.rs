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
    dto::ws_dto::ws_message_dto::WsMessage,
};

pub async fn session_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state.ws_clients))
        .into_response()
}

async fn handle_socket(ws: WebSocket, clients: Clients) {
    static NEXT_ID: Mutex<usize> = Mutex::const_new(0);

    let my_id = {
        let mut id_lock = NEXT_ID.lock().await;
        let id = *id_lock;
        *id_lock += 1;
        id
    };

    let (mut sender_ws, mut receiver_ws) = ws.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    clients.write().await.insert(my_id, tx);

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender_ws.send(msg).await.is_err() {
                debug!("Failed to send message to client, connection closed");
                break;
            }
        }
    });

    let clients_clone = clients.clone();
    let receive_task = tokio::spawn(async move {
        while let Some(result) = receiver_ws.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(ws_msg) => {
                            if let Err(validation_errors) = validate_message(&ws_msg) {
                                error!(
                                    "Validation failed for client {}: {}",
                                    my_id, validation_errors
                                );
                                // TODO send error message to client
                                continue;
                            }

                            broadcast_message(&clients_clone, my_id, &ws_msg).await;
                        }
                        Err(e) => {
                            error!("Failed to parse JSON from client {}: {}", my_id, e);
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    debug!("Client {} requested close", my_id);
                    break;
                }
                Ok(Message::Ping(data)) => {
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

    clients.write().await.remove(&my_id);
    debug!("Client {} disconnected", my_id);
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

/// Send a broadcast mesage to all connected clients
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
