use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::protocol::Message,
};
use uuid::Uuid;

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// WebSocket test client helper
pub struct TestClient {
    pub stream: WsStream,
    pub id: String,
}

impl TestClient {
    /// Creates a new WebSocket client connection
    pub async fn new(base_url: &str) -> Self {
        // FIX: Aggiungi il prefisso /ws al path
        let url = format!("{}/ws/workspace/session", base_url);
        let (stream, _) = connect_async(url).await.expect("Failed to connect");

        Self {
            stream,
            id: format!("client-{}", Uuid::new_v4()),
        }
    }

    /// Sends a JSON message to the server
    pub async fn send(&mut self, msg: Value) {
        let message = Message::Text(msg.to_string().into());
        self.stream.send(message).await.expect("Failed to send");
    }

    /// Receives a JSON message from the server with timeout
    pub async fn recv(&mut self) -> Value {
        match timeout(Duration::from_secs(5), self.stream.next()).await {
            Ok(Some(Ok(Message::Text(text)))) => {
                serde_json::from_str(text.as_ref()).expect("Failed to parse JSON")
            }
            Ok(Some(Ok(_))) => panic!("Expected text message"),
            Ok(Some(Err(e))) => panic!("WebSocket error: {:?}", e),
            Ok(None) => panic!("Connection closed unexpectedly"),
            Err(_) => panic!("Timeout waiting for message (5 seconds)"),
        }
    }

    /// Waits for a success response with the given requestid, skipping broadcasts
    pub async fn expect_success(&mut self, requestid: &str) {
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > Duration::from_secs(10) {
                panic!(
                    "Timeout waiting for success response with requestId: {}",
                    requestid
                );
            }

            let msg = self.recv().await;
            if self.is_broadcast(&msg) {
                continue;
            }

            assert!(
                msg.get("success").is_some(),
                "Expected success response, got {:?}",
                msg
            );
            assert_eq!(
                msg["success"]["requestId"].as_str().unwrap(),
                requestid,
                "Request ID mismatch"
            );
            break;
        }
    }

    /// Waits for a broadcast message of the given type, skipping success responses
    pub async fn expect_broadcast(&mut self, msgtype: &str) -> Value {
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > Duration::from_secs(10) {
                panic!("Timeout waiting for broadcast of type: {}", msgtype);
            }

            let msg = self.recv().await;

            // Skip success and error responses
            if msg.get("success").is_some() || msg.get("error").is_some() {
                continue;
            }

            // Found the message we're looking for
            if msg.get(msgtype).is_some() {
                return msg;
            }

            // Skip other broadcasts
            if self.is_broadcast(&msg) {
                continue;
            }

            panic!("Expected broadcast of type {}, got {:?}", msgtype, msg);
        }
    }

    /// Closes the WebSocket connection
    pub async fn close(&mut self) {
        self.stream.close(None).await.expect("Failed to close");
    }

    /// Checks if a message is a broadcast (not a direct response)
    fn is_broadcast(&self, msg: &Value) -> bool {
        msg.get("updateWorkspace").is_some()
            || msg.get("startSession").is_some()
            || msg.get("noteUpdate").is_some()
            || msg.get("completeSession").is_some()
            || msg.get("endSession").is_some()
    }
}
