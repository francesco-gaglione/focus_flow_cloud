use chrono::{DateTime, Duration, NaiveDate, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum TaskSchedule {
    Unscheduled,
    AllDay {
        date: NaiveDate,
    },
    At {
        starts_at: DateTime<Utc>,
    },
    Span {
        starts_at: DateTime<Utc>,
        duration: Duration,
    },
}

impl TaskSchedule {
    pub fn is_overdue(&self) -> bool {
        let now = Utc::now();

        match self {
            TaskSchedule::Unscheduled => false,
            TaskSchedule::AllDay { date } => date < &now.date_naive(),
            TaskSchedule::At { starts_at } => starts_at < &now,
            TaskSchedule::Span {
                starts_at,
                duration,
            } => (*starts_at + *duration) < now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_overdue() {
        let now = Utc::now();

        assert!(!TaskSchedule::Unscheduled.is_overdue());
        assert!(!TaskSchedule::AllDay {
            date: now.date_naive()
        }
        .is_overdue());
        assert!(!TaskSchedule::At { starts_at: now }.is_overdue());
        assert!(!TaskSchedule::Span {
            starts_at: now,
            duration: Duration::days(1),
        }
        .is_overdue());
    }
}
