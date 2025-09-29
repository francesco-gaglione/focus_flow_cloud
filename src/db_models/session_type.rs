use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

impl SessionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SessionType::Work => "work",
            SessionType::ShortBreak => "short_break",
            SessionType::LongBreak => "long_break",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "work" => Some(SessionType::Work),
            "short_break" => Some(SessionType::ShortBreak),
            "long_break" => Some(SessionType::LongBreak),
            _ => None,
        }
    }
}
