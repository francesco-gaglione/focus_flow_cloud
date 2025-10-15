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
        let test_db_url = base_url.replace("focusflow", "focusflow_test");
        unsafe {
            std::env::set_var("DATABASE_URL", test_db_url);
        }
    });
}

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
async fn test_validation_error() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    client
        .send(json!({
            "requestId": "req-error",
            "completeSession": {
                "concentrationScore": 15, // Invalid: max is 10
                "actualDuration": 1500
            }
        }))
        .await;

    let error_response = client.recv().await;
    assert!(error_response.get("error").is_some());
    assert_eq!(
        error_response["error"]["code"].as_str().unwrap(),
        "VALIDATION_ERROR"
    );
    assert_eq!(
        error_response["error"]["requestId"].as_str().unwrap(),
        "req-error"
    );
}

#[tokio::test]
async fn test_pomodoro_workflow_suggests_short_break() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    // Start work session
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
    client.expect_success("req-001").await;

    // Complete work session
    client
        .send(json!({
            "requestId": "req-002",
            "completeSession": {
                "concentrationScore": 8,
                "actualDuration": 1500
            }
        }))
        .await;
    client.expect_success("req-002").await;

    // IMPORTANT: Consume the completeSession broadcast first
    let complete_broadcast = client.expect_broadcast("completeSession").await;
    assert_eq!(
        complete_broadcast["completeSession"]["concentrationScore"]
            .as_i64()
            .unwrap(),
        8
    );

    // Now get the suggested session
    let suggested = client.expect_broadcast("startSession").await;
    assert_eq!(
        suggested["startSession"]["sessionType"].as_str().unwrap(),
        "ShortBreak"
    );
}

#[tokio::test]
async fn test_end_session_resets_state() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    // Start session
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
    client.expect_success("req-001").await;

    // Complete session
    client
        .send(json!({
            "requestId": "req-002",
            "completeSession": {
                "concentrationScore": 8,
                "actualDuration": 1500
            }
        }))
        .await;
    client.expect_success("req-002").await;

    // IMPORTANT: Consume both broadcasts
    client.expect_broadcast("completeSession").await;
    client.expect_broadcast("startSession").await;

    // End session
    client
        .send(json!({
            "requestId": "req-003",
            "endSession": null
        }))
        .await;
    client.expect_success("req-003").await;
}

#[tokio::test]
async fn test_cannot_start_session_when_one_is_running() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    // Start first session
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
    client.expect_success("req-001").await;

    client.expect_broadcast("startSession").await;

    // Try to start second session (should fail)
    client
        .send(json!({
            "requestId": "req-002",
            "startSession": {
                "sessionType": "Work",
                "startDate": 1728936500,
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": null
            }
        }))
        .await;

    let error_response = client.recv().await;
    assert!(error_response.get("error").is_some());
    assert!(
        error_response["error"]["message"]
            .as_str()
            .unwrap()
            .contains("already running")
    );
}

#[tokio::test]
async fn test_update_note_during_session() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    // Start session
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
    client.expect_success("req-001").await;

    // Update note
    client
        .send(json!({
            "requestId": "req-002",
            "noteUpdate": {
                "newNote": "Making progress on the feature"
            }
        }))
        .await;
    client.expect_success("req-002").await;
}

#[tokio::test]
async fn test_sync_workspace() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    // First, update workspace with category and task
    client
        .send(json!({
            "requestId": "req-000",
            "updateWorkspace": {
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": "223e4567-e89b-12d3-a456-426614174000"
            }
        }))
        .await;

    client.expect_success("req-000").await;
    client.expect_broadcast("updateWorkspace").await;

    // Start a session
    client
        .send(json!({
            "requestId": "req-001",
            "startSession": {
                "sessionType": "Work",
                "startDate": 1728936000,
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": "223e4567-e89b-12d3-a456-426614174000"
            }
        }))
        .await;

    client.expect_success("req-001").await;
    client.expect_broadcast("startSession").await;

    // Update note during session
    client
        .send(json!({
            "requestId": "req-002",
            "noteUpdate": {
                "newNote": "Working on feature X"
            }
        }))
        .await;

    client.expect_success("req-002").await;
    client.expect_broadcast("noteUpdate").await;

    // Request sync
    client
        .send(json!({
            "requestId": "req-003",
            "requestSync": null
        }))
        .await;

    let sync_response = client.recv().await;

    // Verify sync response structure
    assert!(
        sync_response.get("sync").is_some(),
        "Expected 'sync' field in response: {:?}",
        sync_response
    );

    let sync = &sync_response["sync"];

    // Verify session data
    assert!(
        sync.get("session").is_some() && !sync["session"].is_null(),
        "Expected session data"
    );
    let session = &sync["session"];
    assert_eq!(session["sessionType"].as_str().unwrap(), "Work");
    assert_eq!(session["startDate"].as_i64().unwrap(), 1728936000);

    // Check note
    if let Some(note) = session.get("note") {
        if !note.is_null() {
            assert_eq!(note.as_str().unwrap(), "Working on feature X");
        }
    }

    // Verify workspace data (from updateWorkspace, not from startSession)
    assert_eq!(
        sync["categoryId"].as_str().unwrap(),
        "123e4567-e89b-12d3-a456-426614174000"
    );
    assert_eq!(
        sync["taskId"].as_str().unwrap(),
        "223e4567-e89b-12d3-a456-426614174000"
    );
}

#[tokio::test]
async fn test_sync_workspace_no_active_session() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    // Request sync without active session
    client
        .send(json!({
            "requestId": "req-001",
            "requestSync": null
        }))
        .await;

    // Receive the sync response
    let sync_response = client.recv().await;

    // Verify sync response structure
    assert!(
        sync_response.get("sync").is_some(),
        "Expected 'sync' field in response: {:?}",
        sync_response
    );

    let sync = &sync_response["sync"];

    // Should have no session
    assert!(sync["session"].is_null());

    // Workspace should be empty
    assert!(sync["categoryId"].is_null());
    assert!(sync["taskId"].is_null());
}

#[tokio::test]
async fn test_sync_after_workspace_update() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client = TestClient::new(&ws_url).await;

    // Update workspace without starting session
    client
        .send(json!({
            "requestId": "req-001",
            "updateWorkspace": {
                "categoryId": "123e4567-e89b-12d3-a456-426614174000",
                "taskId": "223e4567-e89b-12d3-a456-426614174000"
            }
        }))
        .await;

    // Expect success response for workspace update
    client.expect_success("req-001").await;
    // Consume the broadcast of updateWorkspace
    client.expect_broadcast("updateWorkspace").await;

    // Request sync
    client
        .send(json!({
            "requestId": "req-002",
            "requestSync": null
        }))
        .await;

    // Receive the sync response
    let sync_response = client.recv().await;

    assert!(
        sync_response.get("sync").is_some(),
        "Expected 'sync' field in response: {:?}",
        sync_response
    );
    let sync = &sync_response["sync"];

    // No session should be active
    assert!(sync["session"].is_null());

    // But workspace should have the updated values
    assert_eq!(
        sync["categoryId"].as_str().unwrap(),
        "123e4567-e89b-12d3-a456-426614174000"
    );
    assert_eq!(
        sync["taskId"].as_str().unwrap(),
        "223e4567-e89b-12d3-a456-426614174000"
    );
}

#[tokio::test]
async fn test_multiple_clients_sync_independence() {
    let (ws_url, _handle) = spawn_app().await;
    let mut client1 = TestClient::new(&ws_url).await;
    let mut client2 = TestClient::new(&ws_url).await;

    // Client 1 starts a session
    client1
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

    // Client 1 expects success
    client1.expect_success("req-001").await;

    // Client 2 should receive the broadcast of the session start
    let broadcast = client2.recv().await;
    assert!(broadcast.get("startSession").is_some());

    // Client 2 requests sync
    client2
        .send(json!({
            "requestId": "req-002",
            "requestSync": null
        }))
        .await;

    // Receive the sync response
    let sync_response = client2.recv().await;

    assert!(
        sync_response.get("sync").is_some(),
        "Expected 'sync' field in response: {:?}",
        sync_response
    );
    let sync = &sync_response["sync"];

    // Client 2 should see client 1's session since they share state
    assert!(sync["session"].is_object());
    let session = &sync["session"];
    assert_eq!(session["sessionType"].as_str().unwrap(), "Work");
}
