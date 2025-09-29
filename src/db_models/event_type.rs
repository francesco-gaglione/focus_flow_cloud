use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Start,
    Pause,
    Resume,
    BreakStart,
    BreakEnd,
    Complete,
    Abandon,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Start => "start",
            EventType::Pause => "pause",
            EventType::Resume => "resume",
            EventType::BreakStart => "break_start",
            EventType::BreakEnd => "break_end",
            EventType::Complete => "complete",
            EventType::Abandon => "abandon",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "start" => Some(EventType::Start),
            "pause" => Some(EventType::Pause),
            "resume" => Some(EventType::Resume),
            "break_start" => Some(EventType::BreakStart),
            "break_end" => Some(EventType::BreakEnd),
            "complete" => Some(EventType::Complete),
            "abandon" => Some(EventType::Abandon),
            _ => None,
        }
    }
}
