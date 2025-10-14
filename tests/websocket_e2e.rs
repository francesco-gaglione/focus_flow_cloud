mod common;

use common::TestClient;
use focus_flow_cloud::infra::app::create_app;
use focus_flow_cloud::infra::setup::init_app_state;
use serde_json::json;
use std::sync::Once;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::sleep;

static INIT: Once = Once::new();

fn setup_test_env() {
    INIT.call_once(|| {
        dotenvy::dotenv().ok();

        let base_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        let test_db_url = base_url.replace("/focusflow", "/focusflow_test");
        unsafe { std::env::set_var("DATABASE_URL", test_db_url) };
    });
}

/// Spawns the application server in the background and returns its WebSocket URL
async fn spawn_app() -> (String, tokio::task::JoinHandle<()>) {
    setup_test_env();

    let app_state = init_app_state()
        .await
        .expect("Failed to initialize app state");

    let app = create_app(app_state);

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind");

    let addr = listener.local_addr().expect("Failed to get local address");
    let ws_url = format!("ws://{}", addr);

    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("Server failed");
    });

    sleep(Duration::from_millis(100)).await;

    (ws_url, handle)
}

#[tokio::test]
async fn test_single_client_start_session() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    client
        .send(json!({
            "requestId": "req-001",
            "startSession": {
                "sessionType": "Work",
                "startDate": 1728936000,
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": null
            }
        }))
        .await;

    let response = client.recv().await;

    assert!(response.get("success").is_some());
    assert_eq!(
        response["success"]["message"].as_str().unwrap(),
        "Session started"
    );
    assert_eq!(
        response["success"]["requestId"].as_str().unwrap(),
        "req-001"
    );
}

#[tokio::test]
async fn test_two_clients_broadcast() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client1 = TestClient::new(&ws_url).await;
    let mut client2 = TestClient::new(&ws_url).await;

    client1
        .send(json!({
            "requestId": "req-001",
            "updateWorkspace": {
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": null
            }
        }))
        .await;

    client1.expect_success("req-001").await;

    let broadcast = client2.expect_broadcast("updateWorkspace").await;
    assert_eq!(
        broadcast["updateWorkspace"]["categoryId"].as_str().unwrap(),
        "123e4567-e89b-12d3-a456-426614174000"
    );
    assert!(broadcast["updateWorkspace"]["taskId"].is_null());
}

#[tokio::test]
async fn test_concurrent_sessions_workflow() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client1 = TestClient::new(&ws_url).await;
    let mut client2 = TestClient::new(&ws_url).await;

    client1
        .send(json!({
            "requestId": "req-001",
            "startSession": {
                "sessionType": "Work",
                "startDate": 1728936000,
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": "456e7890-e89b-12d3-a456-426614174111"
            }
        }))
        .await;

    client1.expect_success("req-001").await;

    let broadcast = client2.expect_broadcast("startSession").await;
    assert_eq!(
        broadcast["startSession"]["sessionType"].as_str().unwrap(),
        "Work"
    );

    client2
        .send(json!({
            "requestId": "req-002",
            "noteUpdate": {
                "newNote": "Test note from client 2"
            }
        }))
        .await;

    client2.expect_success("req-002").await;

    let broadcast = client1.expect_broadcast("noteUpdate").await;
    assert_eq!(
        broadcast["noteUpdate"]["newNote"].as_str().unwrap(),
        "Test note from client 2"
    );

    client1
        .send(json!({
            "requestId": "req-003",
            "completeSession": {
                "concentrationScore": 8,
                "actualDuration": 1500
            }
        }))
        .await;

    client1.expect_success("req-003").await;

    let broadcast = client2.expect_broadcast("completeSession").await;
    assert_eq!(
        broadcast["completeSession"]["concentrationScore"]
            .as_i64()
            .unwrap(),
        8
    );
}

#[tokio::test]
async fn test_validation_error() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    client
        .send(json!({
            "requestId": "req-error",
            "completeSession": {
                "concentrationScore": 15,
                "actualDuration": 1500
            }
        }))
        .await;

    let error_response = client.recv().await;
    assert_eq!(
        error_response["error"]["message"].as_str().unwrap(),
        "VALIDATION_ERROR"
    );
    assert_eq!(
        error_response["error"]["requestId"].as_str().unwrap(),
        "req-error"
    );
}

#[tokio::test]
async fn test_multiple_concurrent_clients() {
    let (ws_url, _handle) = spawn_app().await;

    let mut clients = vec![];
    for _ in 0..5 {
        let client = TestClient::new(&ws_url).await;
        clients.push(client);
    }

    clients[0]
        .send(json!({
            "requestId": "req-multi",
            "updateWorkspace": {
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": null
            }
        }))
        .await;

    clients[0].expect_success("req-multi").await;

    for i in 1..5 {
        let broadcast = clients[i].expect_broadcast("updateWorkspace").await;
        assert_eq!(
            broadcast["updateWorkspace"]["categoryId"].as_str().unwrap(),
            "123e4567-e89b-12d3-a456-426614174000"
        );
    }
}

#[tokio::test]
async fn test_client_disconnect() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client1 = TestClient::new(&ws_url).await;
    let mut client2 = TestClient::new(&ws_url).await;

    client1.close().await;

    client2
        .send(json!({
            "requestId": "req-disconnect",
            "updateWorkspace": {
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": null
            }
        }))
        .await;

    client2.expect_success("req-disconnect").await;
}

#[tokio::test]
async fn test_session_lifecycle_complete() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    client
        .send(json!({
            "requestId": "req-001",
            "updateWorkspace": {
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": "456e7890-e89b-12d3-a456-426614174111"
            }
        }))
        .await;

    client.expect_success("req-001").await;

    client
        .send(json!({
            "requestId": "req-002",
            "startSession": {
                "sessionType": "Work",
                "startDate": 1728936000,
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": "456e7890-e89b-12d3-a456-426614174111"
            }
        }))
        .await;

    client.expect_success("req-002").await;

    client
        .send(json!({
            "requestId": "req-003",
            "noteUpdate": {
                "newNote": "Working on implementation"
            }
        }))
        .await;

    loop {
        let msg = client.recv().await;

        if msg.get("updateWorkspace").is_some()
            || msg.get("startSession").is_some()
            || msg.get("noteUpdate").is_some()
            || msg.get("completeSession").is_some()
            || msg.get("endSession").is_some()
        {
            continue;
        }

        if msg.get("success").is_some()
            && msg["success"]["requestId"].as_str().unwrap() == "req-003"
        {
            break;
        }
    }

    client
        .send(json!({
            "requestId": "req-004",
            "completeSession": {
                "concentrationScore": 9,
                "actualDuration": 1500
            }
        }))
        .await;

    client.expect_success("req-004").await;
}

#[tokio::test]
async fn test_invalid_session_type() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    client
        .send(json!({
            "requestId": "req-invalid",
            "startSession": {
                "sessionType": "invalidType",
                "startDate": 1728936000,
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": null
            }
        }))
        .await;

    let error_response = client.recv().await;
    assert!(error_response.get("error").is_some());
}
