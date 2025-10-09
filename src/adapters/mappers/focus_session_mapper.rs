use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    adapters::http::dto::session_api::{
        create_manual_session::CreateManualSessionDto, session_type_enum::SessionTypeEnum,
    },
    application::{
        app_error::{AppError, AppResult},
        use_cases::commands::create_manual_session::CreateManualFocusSessionCommand,
    },
    domain::entities::focus_session_type::FocusSessionType,
};

/// Mapper for FocusSession-related conversions between HTTP layer and Application layer
pub struct FocusSessionMapper;

impl FocusSessionMapper {
    /// Convert CreateManualSessionDto to CreateManualFocusSessionCommand
    pub fn manual_create_dto_to_command(
        dto: &CreateManualSessionDto,
    ) -> AppResult<CreateManualFocusSessionCommand> {
        Ok(CreateManualFocusSessionCommand {
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

    pub fn session_type_dto_to_focus_session_type(dto: SessionTypeEnum) -> FocusSessionType {
        match dto {
            SessionTypeEnum::Work => FocusSessionType::Work,
            SessionTypeEnum::ShortBreak => FocusSessionType::ShortBreak,
            SessionTypeEnum::LongBreak => FocusSessionType::LongBreak,
        }
    }

    // Private helper methods for timestamp conversions

    /// Convert Unix timestamp (seconds) to DateTime<Utc>
    fn timestamp_to_datetime(timestamp: i64) -> AppResult<DateTime<Utc>> {
        let datetime = DateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| AppError::BadRequest(format!("Invalid timestamp: {}", timestamp)))?;
        Ok(datetime)
    }
}
