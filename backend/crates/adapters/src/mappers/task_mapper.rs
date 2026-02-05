use crate::http::dto::common::task_dto::TaskDto;
use crate::http::task::create_task::CreateTaskDto;
use crate::http::task::update_task::UpdateTaskDto;
use application::use_cases::task::command::create_task::CreateTaskCommand;
use application::use_cases::task::command::update_task::UpdateTaskCommand;
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use domain::entities::task::Task;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum TaskMapperError {
    #[error("Invalid category UUID: {0}")]
    InvalidCategoryUuid(uuid::Error),
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(i64),
}

pub type TaskMapperResult<T> = Result<T, TaskMapperError>;

/// Mapper for Task-related conversions between HTTP layer and Application layer
pub struct TaskMapper;

impl TaskMapper {
    /// Convert CreateTaskDto to CreateTaskCommand
    /// Handles timestamp conversion and UUID parsing
    pub fn create_dto_to_command(
        user_id: Uuid,
        dto: CreateTaskDto,
    ) -> TaskMapperResult<CreateTaskCommand> {
        let scheduled_date = Self::timestamp_to_naive_date(dto.scheduled_date)?;

        let category_id = dto
            .category_id
            .map(|id| Uuid::parse_str(&id))
            .transpose()
            .map_err(TaskMapperError::InvalidCategoryUuid)?;

        Ok(CreateTaskCommand {
            user_id,
            name: dto.name,
            description: dto.description,
            category_id,
            scheduled_date,
        })
    }

    /// Convert UpdateTaskDto to UpdateTaskCommand
    /// Handles timestamp conversions and UUID parsing
    pub fn update_dto_to_command(
        task_id: Uuid,
        dto: UpdateTaskDto,
    ) -> TaskMapperResult<UpdateTaskCommand> {
        let category_id = dto
            .category_id
            .map(|id| Uuid::parse_str(&id))
            .transpose()
            .map_err(TaskMapperError::InvalidCategoryUuid)?;

        let scheduled_date = Self::timestamp_to_naive_date(dto.scheduled_date)?;
        let completed_at = Self::timestamp_to_datetime(dto.completed_at)?;

        Ok(UpdateTaskCommand {
            id: task_id,
            category_id,
            name: dto.name,
            description: dto.description,
            scheduled_date,
            completed_at,
        })
    }

    /// Convert domain Task entity to TaskDto for HTTP responses
    pub fn entity_to_dto(task: Task) -> TaskDto {
        TaskDto {
            id: task.id().to_string(),
            category_id: task.category_id().map(|id| id.to_string()),
            name: task.name().to_string(),
            description: task.description().map(|s| s.to_string()),
            scheduled_date: Self::naive_date_to_timestamp(task.scheduled_date()),
            completed_at: task.completed_at().map(|dt| dt.timestamp()),
        }
    }

    /// Convert Vec<Task> to Vec<TaskDto>
    pub fn entities_to_dtos(tasks: Vec<Task>) -> Vec<TaskDto> {
        tasks.into_iter().map(Self::entity_to_dto).collect()
    }

    // Private helper methods for timestamp conversions

    /// Convert Unix timestamp (seconds) to NaiveDate
    fn timestamp_to_naive_date(timestamp: Option<i64>) -> TaskMapperResult<Option<NaiveDate>> {
        match timestamp {
            Some(ts) => {
                let datetime = DateTime::from_timestamp(ts, 0)
                    .ok_or_else(|| TaskMapperError::InvalidTimestamp(ts))?;
                Ok(Some(datetime.date_naive()))
            }
            None => Ok(None),
        }
    }

    /// Convert Unix timestamp (seconds) to DateTime<Utc>
    fn timestamp_to_datetime(timestamp: Option<i64>) -> TaskMapperResult<Option<DateTime<Utc>>> {
        match timestamp {
            Some(ts) => {
                let datetime = DateTime::from_timestamp(ts, 0)
                    .ok_or_else(|| TaskMapperError::InvalidTimestamp(ts))?;
                Ok(Some(datetime))
            }
            None => Ok(None),
        }
    }

    /// Convert NaiveDate to Unix timestamp (seconds)
    /// Uses midnight UTC as the time component
    fn naive_date_to_timestamp(date: Option<NaiveDate>) -> Option<i64> {
        date.map(|d| {
            d.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .and_utc()
                .timestamp()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_timestamp_to_naive_date_valid() {
        let timestamp = 1696550400; // 2023-10-06 00:00:00 UTC
        let result = TaskMapper::timestamp_to_naive_date(Some(timestamp)).unwrap();
        assert!(result.is_some());
        let date = result.unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 10);
        assert_eq!(date.day(), 6);
    }

    #[test]
    fn test_timestamp_to_naive_date_none() {
        let result = TaskMapper::timestamp_to_naive_date(None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_timestamp_to_naive_date_invalid() {
        let timestamp = i64::MAX; // Invalid timestamp
        let result = TaskMapper::timestamp_to_naive_date(Some(timestamp));
        assert!(result.is_err());
    }

    #[test]
    fn test_naive_date_to_timestamp() {
        let date = NaiveDate::from_ymd_opt(2023, 10, 6).unwrap();
        let timestamp = TaskMapper::naive_date_to_timestamp(Some(date));
        assert!(timestamp.is_some());
        assert_eq!(timestamp.unwrap(), 1696550400);
    }
}
