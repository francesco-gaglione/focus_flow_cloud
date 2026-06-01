mod common;

use adapters::http::dto::common::task_dto::TaskScheduleDto;
use adapters::http::task::create_task::CreateTaskDto;
use adapters::http::task::get_tasks::TasksResponseDto;
use adapters::http::task::update_task::{UpdateTaskDto, UpdateTaskResponseDto};
use adapters::http::{
    category::create_category::CreateCategoryDto, task::delete_tasks::DeleteTaskResponseDto,
};
use chrono::Utc;

use crate::common::setup;

#[tokio::test]
async fn create_new_task_and_list() {
    let context = setup().await;

    // Create Category
    let create_category_dto = CreateCategoryDto {
        name: "Work".to_string(),
        color: Some("#FF5733".to_string()),
    };
    context.create_category(&create_category_dto).await;

    // Create Task
    let create_task_dto = CreateTaskDto {
        title: "Task".to_string(),
        description: Some("Work related tasks".to_string()),
        schedule: None,
        subtasks: None,
        category_id: None,
        priority: None,
    };

    let create_task_body = context.create_task(&create_task_dto).await;

    let response = context
        .client
        .get(format!("{}/api/task", context.base_url))
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
    assert!(body.tasks.iter().any(|t| t.title.eq("Task")));
}

#[tokio::test]
async fn create_new_orphan_and_list() {
    let context = setup().await;

    // Create Task without category
    let create_task_dto = CreateTaskDto {
        title: "Orphan".to_string(),
        description: Some("Work related tasks".to_string()),
        schedule: None,
        subtasks: None,
        category_id: None,
        priority: None,
    };

    let create_task_body = context.create_task(&create_task_dto).await;

    let response = context
        .client
        .get(format!("{}/api/task", context.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
    let body: TasksResponseDto = response
        .json()
        .await
        .expect("Failed to deserialize response");

    let orphans: Vec<_> = body
        .tasks
        .iter()
        .filter(|t| t.category_id.is_none())
        .collect();

    assert_eq!(orphans.len(), 1);
    assert!(orphans.iter().any(|t| t.id.eq(&create_task_body.id)));
    assert!(orphans.iter().any(|t| t.title.eq("Orphan")));
}

#[tokio::test]
async fn create_scheduled_task_and_list() {
    let context = setup().await;

    let now = Utc::now();
    let due = now.date_naive().and_hms_opt(11, 0, 0).unwrap().and_utc();

    let create_task_dto = CreateTaskDto {
        title: "Scheduled Task".to_string(),
        description: Some("Description".to_string()),
        schedule: Some(TaskScheduleDto::At {
            starts_at: due.timestamp(),
        }),
        subtasks: None,
        category_id: None,
        priority: None,
    };

    context.create_task(&create_task_dto).await;

    let tasks_res = context
        .client
        .get(format!("{}/api/task?completed=false", context.base_url))
        .send()
        .await
        .expect("Failed to fetch task");

    let tasks_body: TasksResponseDto = tasks_res.json().await.expect("Failed to deserialize tasks");
    assert_eq!(tasks_body.tasks.len(), 1);
    let task = tasks_body.tasks.first().unwrap();
    match &task.schedule {
        TaskScheduleDto::At { starts_at } => assert_eq!(*starts_at, due.timestamp()),
        _ => panic!("Expected At schedule"),
    }
}

#[tokio::test]
async fn update_task_test() {
    let context = setup().await;

    // Create Task
    let create_task_dto = CreateTaskDto {
        title: "Task to update".to_string(),
        description: Some("Description".to_string()),
        schedule: None,
        subtasks: None,
        category_id: None,
        priority: None,
    };

    let create_body = context.create_task(&create_task_dto).await;
    let task_id = create_body.id;

    // Update Task title
    let update_dto = UpdateTaskDto {
        title: Some("Updated Task Name".to_string()),
        description: Some("Updated Description".to_string()),
        schedule: None,
        completed: None,
        priority: None,
    };

    let update_res = context
        .client
        .patch(format!("{}/api/task/{}", context.base_url, task_id))
        .json(&update_dto)
        .send()
        .await
        .expect("Failed to update task");

    assert_eq!(update_res.status(), 200);
    let update_body: UpdateTaskResponseDto = update_res
        .json()
        .await
        .expect("Failed to deserialize update response");

    assert_eq!(update_body.success, true);

    // Complete the task via PATCH with completed flag
    let complete_dto = UpdateTaskDto {
        title: None,
        description: None,
        schedule: None,
        completed: Some(true),
        priority: None,
    };

    let complete_res = context
        .client
        .patch(format!("{}/api/task/{}", context.base_url, task_id))
        .json(&complete_dto)
        .send()
        .await
        .expect("Failed to complete task");

    assert_eq!(complete_res.status(), 200);

    let tasks_res = context
        .client
        .get(format!("{}/api/task?completed=true", context.base_url))
        .send()
        .await
        .expect("Failed to fetch task");

    let tasks_body: TasksResponseDto = tasks_res.json().await.expect("Failed to deserialize tasks");
    assert_eq!(tasks_body.tasks.len(), 1);
}

#[tokio::test]
async fn delete_tasks_test() {
    let context = setup().await;

    // Create Task
    let create_task_dto = CreateTaskDto {
        title: "Task to delete".to_string(),
        description: None,
        schedule: None,
        subtasks: None,
        category_id: None,
        priority: None,
    };

    let create_body = context.create_task(&create_task_dto).await;
    let task_id = create_body.id;

    // Delete Task via query param
    let delete_res = context
        .client
        .delete(format!("{}/api/task", context.base_url))
        .query(&[("taskId", task_id.as_str())])
        .send()
        .await
        .expect("Failed to delete task");

    assert_eq!(delete_res.status(), 200);
    let delete_body: DeleteTaskResponseDto = delete_res
        .json()
        .await
        .expect("Failed to deserialize delete response");
    assert_eq!(delete_body.deleted_id, task_id);

    // Verify it's gone
    let list_res = context
        .client
        .get(format!("{}/api/task", context.base_url))
        .send()
        .await
        .expect("Failed to list tasks");

    let list_body: TasksResponseDto = list_res.json().await.expect("Failed to deserialize list");
    assert!(!list_body.tasks.iter().any(|t| t.id == task_id));
}
