use dioxus::{
    logger::tracing::debug,
    signals::{ReadableExt, Signal},
};
use shared::task::{
    CreateSubtaskDto, CreateSubtaskResponseDto, CreateTaskDto, CreateTaskResponseDto,
    DeleteTaskResponseDto, TaskPriority, TaskScheduleDto, TasksResponseDto, UpdateSubTaskDto,
    UpdateSubTaskResponseDto, UpdateTaskDto, UpdateTaskResponseDto,
};

use crate::clients::http_client::{ApiClient, ApiError, ApiResult};

pub async fn get_all_tasks() -> ApiResult<TasksResponseDto> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    api.get("/api/task", None, None).await
}

pub async fn create_task(body: CreateTaskDto) -> ApiResult<String> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let response: CreateTaskResponseDto = api.post("/api/task", None, None, &body).await?;
    Ok(response.id)
}

pub async fn delete_task(task_id: &str) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let _: DeleteTaskResponseDto = api
        .delete("/api/task", None, Some(&[("taskId", task_id)]))
        .await?;
    Ok(())
}

pub async fn update_task(
    task_id: &str,
    title: Option<String>,
    description: Option<String>,
    schedule: Option<TaskScheduleDto>,
    completed: Option<bool>,
    priority: Option<TaskPriority>,
) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let dto = UpdateTaskDto {
        title,
        description,
        schedule,
        completed,
        priority,
    };
    debug!("Updating task: {:?}", dto);
    let _ = api
        .patch::<UpdateTaskDto, UpdateTaskResponseDto>(
            &format!("/api/task/{}", task_id),
            None,
            None,
            &dto,
        )
        .await?;
    Ok(())
}

pub async fn create_subtask(
    task_id: &str,
    title: String,
    description: Option<String>,
) -> ApiResult<String> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let body = CreateSubtaskDto { title, description };
    let response: CreateSubtaskResponseDto = api
        .post(&format!("/api/task/{}/subtask", task_id), None, None, &body)
        .await?;
    Ok(response.id)
}

pub async fn udpate_subtask(
    task_id: &str,
    subtask_id: &str,
    completed: Option<bool>,
) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let delelete_task_dto = UpdateSubTaskDto { completed };
    debug!("Completing subtask: {:?}", delelete_task_dto);
    let _ = api
        .patch::<UpdateSubTaskDto, UpdateSubTaskResponseDto>(
            &format!("/api/task/{}/subtask/{}", task_id, subtask_id),
            None,
            None,
            &delelete_task_dto,
        )
        .await?;
    Ok(())
}
