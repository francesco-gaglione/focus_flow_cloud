use domain::tasks::entities::focus_session_type::FocusSessionType;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use application::tasks::use_cases::pomodoro_state::fetch_user_pomodoro_state::UserSessionType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
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

pub fn domain_to_enum(v: FocusSessionType) -> SessionTypeEnum {
    match v {
        FocusSessionType::Work => SessionTypeEnum::Work,
        FocusSessionType::ShortBreak => SessionTypeEnum::ShortBreak,
        FocusSessionType::LongBreak => SessionTypeEnum::LongBreak,
    }
}

pub fn enum_to_domain(v: SessionTypeEnum) -> FocusSessionType {
    match v {
        SessionTypeEnum::Work => FocusSessionType::Work,
        SessionTypeEnum::ShortBreak => FocusSessionType::ShortBreak,
        SessionTypeEnum::LongBreak => FocusSessionType::LongBreak,
    }
}

pub fn app_type_to_enum(v: UserSessionType) -> SessionTypeEnum {
    match v {
        UserSessionType::Work => SessionTypeEnum::Work,
        UserSessionType::ShortBreak => SessionTypeEnum::ShortBreak,
        UserSessionType::LongBreak => SessionTypeEnum::LongBreak,
    }
}
