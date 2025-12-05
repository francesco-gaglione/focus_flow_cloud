mod common;

use api::adapters::http::dto::{
    category_api::create_category::CreateCategoryDto,
    common::session_type_enum::SessionTypeEnum,
    session_api::{
        create_manual_session::CreateManualSessionDto, get_sessions::GetSessionFiltersResponseDto,
        update_session::UpdateFocusSessionDto,
    },
    task_api::{create_task::CreateTaskDto, get_tasks::TasksResponseDto},
};
use chrono::Utc;
use tracing::info;

use crate::common::setup;

#[tokio::test]
async fn create_new_session_and_list() {
    let context = setup().await;

    // Create Category to link to the task
    let create_category_dto = CreateCategoryDto {
        name: "Work".to_string(),
        description: Some("Work related tasks".to_string()),
        color: Some("#FF5733".to_string()),
    };
    let category_body = context.create_category(&create_category_dto).await;

    // Create Task
    let create_task_dto = CreateTaskDto {
        name: "Task".to_string(),
        description: Some("Work related tasks".to_string()),
        category_id: Some(category_body.category_id.clone()),
        scheduled_date: Some(chrono::Utc::now().timestamp()),
    };

    let create_task_body = context.create_task(&create_task_dto).await;

    // Fetch tasks and check if the task was created
    let response = context
        .client
        .get(format!("{}/api/tasks", context.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body: TasksResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");
    assert!(body.tasks.len() == 1);
    assert!(body
        .tasks
        .iter()
        .any(|t| t.id.eq(&create_task_body.id.clone())));
    assert!(body.tasks.iter().any(|t| t.name.eq("Task")));

    // Create manual work session
    let create_manual_session_dto = CreateManualSessionDto {
        task_id: Some(create_task_body.id.clone()),
        category_id: Some(category_body.category_id.clone()),
        session_type: SessionTypeEnum::Work,
        concentration_score: Some(1),
        started_at: chrono::Utc::now().timestamp(),
        ended_at: chrono::Utc::now().timestamp() + 3600,
        notes: Some("Work session notes".to_string()),
    };
    let create_manual_session_response = context
        .create_manual_session(&create_manual_session_dto)
        .await;

    assert!(create_manual_session_response.id.len() > 0);

    // Fetch sessions and check if the session was created
    let response = context
        .client
        .get(format!("{}/api/focus-sessions", context.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body: GetSessionFiltersResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");
    assert!(body.focus_sessions.len() == 1);
    assert!(body
        .focus_sessions
        .iter()
        .any(|s| s.id.eq(&create_manual_session_response.id)));
    assert!(body
        .focus_sessions
        .iter()
        .any(|s| s.task_id.eq(&Some(create_task_body.id.clone()))));
    assert!(body
        .focus_sessions
        .iter()
        .any(|s| s.category_id.eq(&Some(category_body.category_id.clone()))));
    assert!(body
        .focus_sessions
        .iter()
        .any(|s| s.session_type.eq(&SessionTypeEnum::Work)));
    assert!(body
        .focus_sessions
        .iter()
        .any(|s| s.concentration_score.eq(&Some(1))));
    assert!(body
        .focus_sessions
        .iter()
        .any(|s| s.started_at.eq(&create_manual_session_dto.started_at)));
    assert!(body
        .focus_sessions
        .iter()
        .any(|s| s.ended_at.eq(&Some(create_manual_session_dto.ended_at))));
    assert!(body
        .focus_sessions
        .iter()
        .any(|s| s.notes.eq(&create_manual_session_dto.notes)));
}

#[tokio::test]
async fn update_session_and_list() {
    let context = setup().await;

    // Create Category to link to the task
    let create_category_dto = CreateCategoryDto {
        name: "Work".to_string(),
        description: Some("Work related tasks".to_string()),
        color: Some("#FF5733".to_string()),
    };
    let category_body = context.create_category(&create_category_dto).await;

    let create_category_dto = CreateCategoryDto {
        name: "Study".to_string(),
        description: Some("Study related tasks".to_string()),
        color: Some("#FF5734".to_string()),
    };
    let category_body_2 = context.create_category(&create_category_dto).await;

    // Create Task
    let create_task_dto = CreateTaskDto {
        name: "Task".to_string(),
        description: Some("Work related tasks".to_string()),
        category_id: Some(category_body.category_id.clone()),
        scheduled_date: Some(chrono::Utc::now().timestamp()),
    };

    let create_task_body = context.create_task(&create_task_dto).await;

    let create_task_dto = CreateTaskDto {
        name: "Task".to_string(),
        description: Some("Work related tasks".to_string()),
        category_id: Some(category_body.category_id.clone()),
        scheduled_date: Some(chrono::Utc::now().timestamp()),
    };

    let create_task_body_2 = context.create_task(&create_task_dto).await;

    // Create Manual Session
    let create_manual_session_dto = CreateManualSessionDto {
        task_id: Some(create_task_body.id.clone()),
        category_id: Some(category_body.category_id.clone()),
        session_type: SessionTypeEnum::Work,
        concentration_score: Some(1),
        started_at: chrono::Utc::now().timestamp(),
        ended_at: chrono::Utc::now().timestamp() + 3600,
        notes: Some("Work session notes".to_string()),
    };
    let create_manual_session_response = context
        .create_manual_session(&create_manual_session_dto)
        .await;

    assert!(create_manual_session_response.id.len() > 0);

    // Update Manual Session
    let update_manual_session_dto = UpdateFocusSessionDto {
        category_id: Some(category_body_2.category_id.clone()),
        task_id: Some(create_task_body_2.id.clone()),
        concentration_score: Some(2),
        started_at: Some(Utc::now().timestamp()),
        ended_at: Some(Utc::now().timestamp() + 7200),
        notes: Some("Notes updated".to_string()),
    };

    let response = context
        .client
        .put(format!(
            "{}/api/focus-sessions/{}",
            context.base_url, create_manual_session_response.id
        ))
        .json(&update_manual_session_dto)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);

    // Fetch session and verify updated
    let response = context
        .client
        .get(format!("{}/api/focus-sessions", context.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body: GetSessionFiltersResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");
    assert!(body.focus_sessions.len() == 1);
    let session = body.focus_sessions.first().unwrap();
    info!("Focus sessions: {:?}", body.focus_sessions);
    assert_eq!(session.notes, Some("Notes updated".to_string()));
    assert_eq!(session.category_id, Some(category_body_2.category_id));
    assert_eq!(session.task_id, Some(create_task_body_2.id));
    assert_eq!(session.concentration_score, Some(2));
    assert_eq!(session.actual_duration, Some(7200));
}
