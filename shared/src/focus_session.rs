use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct FocusSessionDto {
    pub id: String,
    pub category_id: Option<String>,
    pub task_id: Option<String>,
    pub session_type: SessionTypeEnum,
    pub actual_duration: Option<i64>,
    pub concentration_score: Option<i32>,
    pub notes: Option<String>,
    pub started_at: i64,
    pub ended_at: Option<i64>,
}
