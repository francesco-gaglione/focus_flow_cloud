use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "work" => Some(SessionTypeEnum::Work),
            "short_break" => Some(SessionTypeEnum::ShortBreak),
            "long_break" => Some(SessionTypeEnum::LongBreak),
            _ => None,
        }
    }
}
