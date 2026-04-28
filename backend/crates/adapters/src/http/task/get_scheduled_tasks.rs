use crate::http::dto::common::task_dto::TaskDto;
use crate::http_error::{map_persistence_error, HttpResult};
use crate::openapi::TASK_TAG;
use crate::{http::app_state::AppState, http_error::HttpError};
use application::use_cases::task::get_scheduled_tasks::{
    GetScheduledTasksError, GetScheduledTasksUseCaseCommand, ScheduledTaskOutput,
};
use axum::extract::{Query, State};
use axum::Json;
use chrono::DateTime;
use domain::entities::tasks::task_priority::TaskPriority;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

impl From<GetScheduledTasksError> for HttpError {
    fn from(err: GetScheduledTasksError) -> Self {
        match err {
            GetScheduledTasksError::TaskPersistenceError(e) => map_persistence_error(e),
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct GetScheduledTasksParams {
    pub completed: Option<bool>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledTasksResponseDto {
    pub tasks: Vec<TaskDto>,
}

fn priority_to_str(p: Option<TaskPriority>) -> Option<String> {
    p.map(|p| match p {
        TaskPriority::Low => "low".to_string(),
        TaskPriority::Medium => "medium".to_string(),
        TaskPriority::High => "high".to_string(),
        TaskPriority::Urgent => "urgent".to_string(),
    })
}

fn scheduled_task_to_dto(value: &ScheduledTaskOutput) -> TaskDto {
    TaskDto {
        id: value.id.to_string(),
        title: value.title.clone(),
        description: value.description.clone(),
        priority: priority_to_str(value.priority),
        due_date: value.due_date.map(|d| d.timestamp()),
        completed_at: value.completed_at.map(|d| d.timestamp()),
    }
}

#[utoipa::path(
    get,
    path = "/api/task/scheduled",
    tag = TASK_TAG,
    summary = "Get scheduled tasks",
    params(
        GetScheduledTasksParams
    ),
    responses(
        (status = 200, description = "Scheduled tasks fetched successfully", body = ScheduledTasksResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_scheduled_tasks_api(
    State(state): State<AppState>,
    Query(params): Query<GetScheduledTasksParams>,
) -> HttpResult<Json<ScheduledTasksResponseDto>> {
    let from = params
        .from
        .map(|s| {
            DateTime::from_timestamp(s, 0)
                .ok_or(HttpError::BadRequest("Invalid from data".to_string()))
        })
        .transpose()?;

    let to = params
        .to
        .map(|s| {
            DateTime::from_timestamp(s, 0)
                .ok_or(HttpError::BadRequest("Invalid from data".to_string()))
        })
        .transpose()?;

    let res = state
        .get_scheduled_task_uc
        .execute(GetScheduledTasksUseCaseCommand {
            completed: params.completed,
            from,
            to,
        })
        .await?;
    Ok(Json(ScheduledTasksResponseDto {
        tasks: res.scheduled_tasks.iter().map(scheduled_task_to_dto).collect(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use application::repository_traits::persistence_error::PersistenceError;
    use application::use_cases::task::get_scheduled_tasks::ScheduledTaskOutput;
    use chrono::Utc;
    use uuid::Uuid;

    fn full_output() -> ScheduledTaskOutput {
        let now = Utc::now();
        ScheduledTaskOutput {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            title: "Task".to_string(),
            description: Some("Desc".to_string()),
            priority: Some(TaskPriority::High),
            due_date: Some(now),
            completed_at: Some(now),
        }
    }

    #[test]
    fn test_task_dto_maps_all_fields() {
        let output = full_output();
        let dto = scheduled_task_to_dto(&output);

        assert_eq!(dto.id, output.id.to_string());
        assert_eq!(dto.title, output.title);
        assert_eq!(dto.description, output.description);
        assert_eq!(dto.due_date, output.due_date.map(|d| d.timestamp()));
        assert_eq!(dto.completed_at, output.completed_at.map(|d| d.timestamp()));
        assert_eq!(dto.priority, Some("high".to_string()));
    }

    #[test]
    fn test_task_dto_maps_none_fields() {
        let output = ScheduledTaskOutput {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            title: "Minimal".to_string(),
            description: None,
            priority: None,
            due_date: None,
            completed_at: None,
        };
        let dto = scheduled_task_to_dto(&output);

        assert!(dto.description.is_none());
        assert!(dto.priority.is_none());
        assert!(dto.due_date.is_none());
        assert!(dto.completed_at.is_none());
    }

    #[test]
    fn test_http_error_from_unexpected_persistence_error() {
        let err = GetScheduledTasksError::TaskPersistenceError(PersistenceError::Unexpected(
            "db down".to_string(),
        ));
        assert!(matches!(HttpError::from(err), HttpError::GenericError(_)));
    }

    #[test]
    fn test_http_error_from_not_found_persistence_error() {
        let err = GetScheduledTasksError::TaskPersistenceError(PersistenceError::NotFound(
            "task not found".to_string(),
        ));
        assert!(matches!(HttpError::from(err), HttpError::NotFound(_)));
    }

    #[test]
    fn test_http_error_from_already_exists_persistence_error() {
        let err = GetScheduledTasksError::TaskPersistenceError(PersistenceError::AlreadyExists);
        assert!(matches!(
            HttpError::from(err),
            HttpError::ResourceAlreadyExist(_)
        ));
    }

    #[test]
    fn test_params_all_fields_deserialized() {
        let qs = "from=1000&to=2000&completed=true";
        let params: GetScheduledTasksParams = serde_urlencoded::from_str(qs).unwrap();

        assert_eq!(params.from, Some(1000));
        assert_eq!(params.to, Some(2000));
        assert_eq!(params.completed, Some(true));
    }

    #[test]
    fn test_params_all_fields_none_when_absent() {
        let params: GetScheduledTasksParams = serde_urlencoded::from_str("").unwrap();

        assert!(params.from.is_none());
        assert!(params.to.is_none());
        assert!(params.completed.is_none());
    }

    #[test]
    fn test_params_completed_false() {
        let params: GetScheduledTasksParams =
            serde_urlencoded::from_str("completed=false").unwrap();
        assert_eq!(params.completed, Some(false));
    }
}
