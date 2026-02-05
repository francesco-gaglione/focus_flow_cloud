use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::http::dto::common::{
    focus_session::FocusSessionDto, session_type_enum::SessionTypeEnum,
};
use crate::http::session::{
    create_manual_session::CreateManualSessionDto, update_session::UpdateFocusSessionDto,
};
use application::use_cases::focus_session::command::{
    create_manual_session::CreateManualFocusSessionCommand,
    update_focus_session::UpdateFocusSessionCommand,
};
use domain::entities::{focus_session::FocusSession, focus_session_type::FocusSessionType};

#[derive(Debug, Error)]
pub enum FocusSessionMapperError {
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(i64),
    #[error("Start time has a bad format")]
    InvalidStartTime,
    #[error("End time has bad format")]
    InvalidEndTime,
}

pub type FocusSessionMapperResult<T> = Result<T, FocusSessionMapperError>;

pub struct FocusSessionMapper;

impl FocusSessionMapper {
    pub fn to_dto(session: &FocusSession) -> FocusSessionDto {
        FocusSessionDto {
            id: session.id().to_string(),
            category_id: session.category_id().map(|id| id.to_string()),
            task_id: session.task_id().map(|id| id.to_string()),
            session_type: SessionTypeEnum::from(session.session_type()),
            actual_duration: session.actual_duration(),
            concentration_score: session.concentration_score(),
            notes: session.notes().clone(),
            started_at: session.started_at().timestamp(),
            ended_at: session.ended_at().map(|dt| dt.timestamp()),
            created_at: session.created_at().timestamp(),
        }
    }

    /// Convert CreateManualSessionDto to CreateManualFocusSessionCommand
    pub fn manual_create_dto_to_command(
        user_id: Uuid,
        dto: &CreateManualSessionDto,
    ) -> FocusSessionMapperResult<CreateManualFocusSessionCommand> {
        Ok(CreateManualFocusSessionCommand {
            user_id,
            category_id: dto
                .category_id
                .clone()
                .map(|id| Uuid::parse_str(id.as_str()).unwrap()), // should be safe due to validation
            task_id: dto
                .task_id
                .clone()
                .map(|id| Uuid::parse_str(id.as_str()).unwrap()), // should be safe due to validation
            session_type: Self::session_type_dto_to_focus_session_type(dto.session_type.clone()),
            concentration_score: dto.concentration_score,
            notes: dto.notes.clone(),
            started_at: Self::timestamp_to_datetime(dto.started_at)?,
            ended_at: Self::timestamp_to_datetime(dto.ended_at)?,
        })
    }

    pub fn session_update_dto_to_command(
        id: String,
        dto: &UpdateFocusSessionDto,
    ) -> FocusSessionMapperResult<UpdateFocusSessionCommand> {
        Ok(UpdateFocusSessionCommand {
            session_id: Uuid::parse_str(id.as_str()).unwrap(), // should be safe due to validation
            category_id: dto
                .category_id
                .clone()
                .map(|id| Uuid::parse_str(id.as_str()).unwrap()), // should be safe due to validation
            task_id: dto
                .task_id
                .clone()
                .map(|id| Uuid::parse_str(id.as_str()).unwrap()), // should be safe due to validation
            concentration_score: dto.concentration_score,
            notes: dto.notes.clone(),
            started_at: dto
                .started_at
                .map(|d| {
                    Self::timestamp_to_datetime(d)
                        .map_err(|_| FocusSessionMapperError::InvalidStartTime)
                })
                .transpose()?,
            ended_at: dto
                .ended_at
                .map(|d| {
                    Self::timestamp_to_datetime(d)
                        .map_err(|_| FocusSessionMapperError::InvalidEndTime)
                })
                .transpose()?,
        })
    }

    pub fn session_type_dto_to_focus_session_type(dto: SessionTypeEnum) -> FocusSessionType {
        match dto {
            SessionTypeEnum::Work => FocusSessionType::Work,
            SessionTypeEnum::ShortBreak => FocusSessionType::ShortBreak,
            SessionTypeEnum::LongBreak => FocusSessionType::LongBreak,
        }
    }

    // Private helper methods for timestamp conversions

    /// Convert Unix timestamp (seconds) to DateTime<Utc>
    fn timestamp_to_datetime(timestamp: i64) -> FocusSessionMapperResult<DateTime<Utc>> {
        let datetime = DateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| FocusSessionMapperError::InvalidTimestamp(timestamp))?;
        Ok(datetime)
    }
}
