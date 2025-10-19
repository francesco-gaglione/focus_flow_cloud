use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum FocusSessionType {
    Work,
    ShortBreak,
    LongBreak,
}

impl FocusSessionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FocusSessionType::Work => "work",
            FocusSessionType::ShortBreak => "short_break",
            FocusSessionType::LongBreak => "long_break",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "work" => Some(FocusSessionType::Work),
            "short_break" => Some(FocusSessionType::ShortBreak),
            "long_break" => Some(FocusSessionType::LongBreak),
            _ => None,
        }
    }
}

impl Display for FocusSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
