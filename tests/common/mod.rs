use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::net::TcpStream;
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
        let url = format!("{}/ws/workspace/session", base_url);
        let (stream, _) = connect_async(&url).await.expect("Failed to connect");

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

    /// Receives a JSON message from the server
    pub async fn recv(&mut self) -> Value {
        match self.stream.next().await {
            Some(Ok(Message::Text(text))) => {
                serde_json::from_str(text.as_ref()).expect("Failed to parse JSON")
            }
            _ => panic!("Expected text message"),
        }
    }

    /// Waits for a success response with the given request_id, skipping broadcasts
    pub async fn expect_success(&mut self, request_id: &str) {
        loop {
            let msg = self.recv().await;

            if self.is_broadcast(&msg) {
                continue;
            }

            assert!(
                msg.get("success").is_some(),
                "Expected success response, got: {:?}",
                msg
            );
            assert_eq!(
                msg["success"]["requestId"].as_str().unwrap(),
                request_id,
                "Request ID mismatch"
            );
            break;
        }
    }

    /// Waits for a broadcast message of the given type, skipping success responses
    pub async fn expect_broadcast(&mut self, msg_type: &str) -> Value {
        loop {
            let msg = self.recv().await;

            if msg.get("success").is_some() || msg.get("error").is_some() {
                continue;
            }

            if msg.get(msg_type).is_some() {
                return msg;
            }

            if self.is_broadcast(&msg) {
                continue;
            }

            panic!("Expected broadcast of type: {}, got: {:?}", msg_type, msg);
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
