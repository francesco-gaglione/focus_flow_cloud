mod common;
use focus_flow_cloud::adapters::http::dto::{
    category_api::create_category::{CreateCategoryDto, CreateCategoryResponseDto},
    task_api::{
        create_task::{CreateTaskDto, CreateTaskResponseDto},
        delete_task::{DeleteTasksDto, DeleteTasksResponseDto},
        get_tasks::TasksResponseDto,
        orphan_tasks::OrphanTasksResponseDto,
        update_task::{UpdateTaskDto, UpdateTaskResponseDto},
    },
};

use crate::common::setup;

#[tokio::test]
async fn create_new_task_and_list() {
    let context = setup().await;
    let client = reqwest::Client::new();

    // Create Category to link to the task
    let create_category_dto = CreateCategoryDto {
        name: "Work".to_string(),
        description: Some("Work related tasks".to_string()),
        color: Some("#FF5733".to_string()),
    };
    let response = client
        .post(format!("{}/api/categories", context.base_url))
        .json(&create_category_dto)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body: CreateCategoryResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");
    assert!(body.category_id.len() > 0);

    // Create Task
    let create_task_dto = CreateTaskDto {
        name: "Task".to_string(),
        description: Some("Work related tasks".to_string()),
        category_id: Some(body.category_id),
        scheduled_date: Some(chrono::Utc::now().timestamp()),
    };

    let response = client
        .post(format!("{}/api/tasks", context.base_url))
        .json(&create_task_dto)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let create_task_body: CreateTaskResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");
    assert!(create_task_body.id.len() > 0);

    // Fetch tasks and check if the task was created
    let response = client
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
    assert!(body.tasks.iter().any(|t| t.id.eq(&create_task_body.id)));
    assert!(body.tasks.iter().any(|t| t.name.eq("Task")));
}

#[tokio::test]
async fn create_new_orphan_and_list() {
    let context = setup().await;
    let client = reqwest::Client::new();

    // Create Category to link to the task
    let create_category_dto = CreateCategoryDto {
        name: "Work".to_string(),
        description: Some("Work related tasks".to_string()),
        color: Some("#FF5733".to_string()),
    };
    let response = client
        .post(format!("{}/api/categories", context.base_url))
        .json(&create_category_dto)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body: CreateCategoryResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");
    assert!(body.category_id.len() > 0);

    // Create Task
    let create_task_dto = CreateTaskDto {
        name: "Orphan".to_string(),
        description: Some("Work related tasks".to_string()),
        category_id: None,
        scheduled_date: Some(chrono::Utc::now().timestamp()),
    };

    let response = client
        .post(format!("{}/api/tasks", context.base_url))
        .json(&create_task_dto)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let create_task_body: CreateTaskResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");
    assert!(create_task_body.id.len() > 0);

    // Fetch tasks and check if the task was created
    let response = client
        .get(format!("{}/api/tasks/orphans", context.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body: OrphanTasksResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");
    assert!(body.orphan_tasks.len() == 1);
    assert!(body
        .orphan_tasks
        .iter()
        .any(|t| t.id.eq(&create_task_body.id)));
    assert!(body.orphan_tasks.iter().any(|t| t.name.eq("Orphan")));
}

#[tokio::test]
async fn update_task_test() {
    let context = setup().await;
    let client = reqwest::Client::new();

    // Create Task
    let create_task_dto = CreateTaskDto {
        name: "Task to update".to_string(),
        description: Some("Description".to_string()),
        category_id: None,
        scheduled_date: Some(chrono::Utc::now().timestamp()),
    };

    let create_res = client
        .post(format!("{}/api/tasks", context.base_url))
        .json(&create_task_dto)
        .send()
        .await
        .expect("Failed to create task");

    assert_eq!(create_res.status(), 200);
    let create_body: CreateTaskResponseDto =
        create_res.json().await.expect("Failed to deserialize");
    let task_id = create_body.id;

    // Update Task
    let update_dto = UpdateTaskDto {
        category_id: None,
        name: Some("Updated Task Name".to_string()),
        description: Some("Updated Description".to_string()),
        scheduled_date: None,
        completed_at: Some(chrono::Utc::now().timestamp()),
    };

    let update_res = client
        .put(format!("{}/api/tasks/{}", context.base_url, task_id))
        .json(&update_dto)
        .send()
        .await
        .expect("Failed to update task");

    assert_eq!(update_res.status(), 200);
    let update_body: UpdateTaskResponseDto = update_res
        .json()
        .await
        .expect("Failed to deserialize update response");

    assert_eq!(update_body.updated_task.name, "Updated Task Name");
    assert_eq!(
        update_body.updated_task.description.unwrap(),
        "Updated Description"
    );
    assert!(update_body.updated_task.completed_at.is_some());
}

#[tokio::test]
async fn delete_tasks_test() {
    let context = setup().await;
    let client = reqwest::Client::new();

    // Create Task
    let create_task_dto = CreateTaskDto {
        name: "Task to delete".to_string(),
        description: None,
        category_id: None,
        scheduled_date: Some(chrono::Utc::now().timestamp()),
    };

    let create_res = client
        .post(format!("{}/api/tasks", context.base_url))
        .json(&create_task_dto)
        .send()
        .await
        .expect("Failed to create task");

    assert_eq!(create_res.status(), 200);
    let create_body: CreateTaskResponseDto =
        create_res.json().await.expect("Failed to deserialize");
    let task_id = create_body.id;

    // Delete Task
    let delete_dto = DeleteTasksDto {
        task_ids: vec![task_id.clone()],
    };

    let delete_res = client
        .delete(format!("{}/api/tasks", context.base_url))
        .json(&delete_dto)
        .send()
        .await
        .expect("Failed to delete task");

    assert_eq!(delete_res.status(), 200);
    let delete_body: DeleteTasksResponseDto = delete_res
        .json()
        .await
        .expect("Failed to deserialize delete response");
    assert!(delete_body.deleted_ids.contains(&task_id));

    // Verify it's gone
    let list_res = client
        .get(format!("{}/api/tasks", context.base_url))
        .send()
        .await
        .expect("Failed to list tasks");

    let list_body: TasksResponseDto = list_res.json().await.expect("Failed to deserialize list");
    assert!(!list_body.tasks.iter().any(|t| t.id == task_id));
}
