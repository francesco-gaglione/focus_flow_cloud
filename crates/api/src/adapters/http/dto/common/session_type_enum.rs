use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use domain::entities::focus_session_type::FocusSessionType;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum SessionTypeEnum {
    Work,
    ShortBreak,
    LongBreak,
}

impl SessionTypeEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            SessionTypeEnum::Work => "work",
            SessionTypeEnum::ShortBreak => "short_break",
            SessionTypeEnum::LongBreak => "long_break",
        }
    }

    pub fn from_text(s: &str) -> Option<Self> {
        match s {
            "work" => Some(SessionTypeEnum::Work),
            "short_break" => Some(SessionTypeEnum::ShortBreak),
            "long_break" => Some(SessionTypeEnum::LongBreak),
            _ => None,
        }
    }
}

impl From<FocusSessionType> for SessionTypeEnum {
    fn from(value: FocusSessionType) -> Self {
        match value {
            FocusSessionType::Work => SessionTypeEnum::Work,
            FocusSessionType::ShortBreak => SessionTypeEnum::ShortBreak,
            FocusSessionType::LongBreak => SessionTypeEnum::LongBreak,
        }
    }
}

impl From<SessionTypeEnum> for FocusSessionType {
    fn from(value: SessionTypeEnum) -> Self {
        match value {
            SessionTypeEnum::Work => FocusSessionType::Work,
            SessionTypeEnum::ShortBreak => FocusSessionType::ShortBreak,
            SessionTypeEnum::LongBreak => FocusSessionType::LongBreak,
        }
    }
}
