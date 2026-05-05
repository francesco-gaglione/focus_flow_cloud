use dioxus::{
    logger::tracing::debug,
    signals::{ReadableExt, Signal},
};
use shared::task::{
    CompleteSubTaskDto, CompleteSubTaskResponseDto, CompleteTaskDto, CompleteTaskResponseDto,
    CreateTaskDto, CreateTaskResponseDto, DeleteTaskDto, DeleteTaskResponseDto, TasksResponseDto,
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

pub async fn complete_task(task_id: &str) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let complete_task_dto = CompleteTaskDto {
        task_id: task_id.to_string(),
    };

    debug!("Completing task: {:?}", complete_task_dto);
    let _ = api
        .post::<CompleteTaskDto, CompleteTaskResponseDto>(
            "/api/task/complete",
            None,
            None,
            &complete_task_dto,
        )
        .await?;
    Ok(())
}

pub async fn complete_sub_task(task_id: &str, subtask_id: &str) -> ApiResult<()> {
    let api_signal = dioxus::core::try_consume_context::<Signal<ApiClient>>()
        .ok_or_else(|| ApiError::ClientError("ApiClient signal not found".to_string()))?;
    let api = (*api_signal.read()).clone();
    let delelete_task_dto = CompleteSubTaskDto {
        task_id: task_id.to_string(),
        subtask_id: subtask_id.to_string(),
    };

    debug!("Completing subtask: {:?}", delelete_task_dto);
    let _ = api
        .post::<CompleteSubTaskDto, CompleteSubTaskResponseDto>(
            "/api/task/subtask/complete",
            None,
            None,
            &delelete_task_dto,
        )
        .await?;
    Ok(())
}
