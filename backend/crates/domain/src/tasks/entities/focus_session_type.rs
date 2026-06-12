use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    #[allow(clippy::should_implement_trait)]
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

#[cfg(test)]
mod focus_session_type_tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(FocusSessionType::Work.as_str(), "work");
        assert_eq!(FocusSessionType::ShortBreak.as_str(), "short_break");
        assert_eq!(FocusSessionType::LongBreak.as_str(), "long_break");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            FocusSessionType::from_str("work"),
            Some(FocusSessionType::Work)
        );
        assert_eq!(
            FocusSessionType::from_str("short_break"),
            Some(FocusSessionType::ShortBreak)
        );
        assert_eq!(
            FocusSessionType::from_str("long_break"),
            Some(FocusSessionType::LongBreak)
        );
        assert_eq!(FocusSessionType::from_str("invalid"), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(FocusSessionType::Work.to_string(), "work");
        assert_eq!(FocusSessionType::ShortBreak.to_string(), "short_break");
        assert_eq!(FocusSessionType::LongBreak.to_string(), "long_break");
    }
}
