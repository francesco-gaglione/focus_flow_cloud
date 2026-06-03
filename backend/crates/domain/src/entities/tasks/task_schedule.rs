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

//TODO i vari metodi is_ non stanno funzionando probabilmente in queanto in response le liste sono sempre vuote e non é possibile i task li ho creati, guarda i log per conferma
impl TaskSchedule {
    pub fn is_overdue(&self) -> bool {
        let now = Utc::now();

        match self {
            TaskSchedule::Unscheduled => false,
            TaskSchedule::AllDay { date } => *date < now.date_naive(),
            TaskSchedule::At { starts_at } => *starts_at < now,
            TaskSchedule::Span {
                starts_at,
                duration,
            } => (*starts_at + *duration) < now,
        }
    }

    pub fn is_today(&self) -> bool {
        let now = Utc::now();

        match self {
            TaskSchedule::Unscheduled => false,
            TaskSchedule::AllDay { date } => *date == now.date_naive(),
            TaskSchedule::At { starts_at } => starts_at.date_naive() == now.date_naive(),
            TaskSchedule::Span {
                starts_at,
                duration,
            } => {
                let end = *starts_at + *duration;
                // Checks if the entire span is contained within today
                starts_at.date_naive() == now.date_naive() && end.date_naive() == now.date_naive()
            }
        }
    }

    pub fn is_next_week(&self) -> bool {
        let now = Utc::now();
        let today = now.date_naive();
        let next_week_end = today + Duration::days(7);

        match self {
            TaskSchedule::Unscheduled => false,
            // Excludes today, includes the next 7 days
            TaskSchedule::AllDay { date } => *date > today && *date <= next_week_end,
            TaskSchedule::At { starts_at } => {
                let d = starts_at.date_naive();
                d > today && d <= next_week_end
            }
            TaskSchedule::Span { starts_at, .. } => {
                let d = starts_at.date_naive();
                d > today && d <= next_week_end
            }
        }
    }

    pub fn is_incoming(&self) -> bool {
        let now = Utc::now();
        let next_week_end = now.date_naive() + Duration::days(7);

        match self {
            TaskSchedule::Unscheduled => false,
            // Excludes today and the next 7 days
            TaskSchedule::AllDay { date } => *date > next_week_end,
            TaskSchedule::At { starts_at } => starts_at.date_naive() > next_week_end,
            TaskSchedule::Span { starts_at, .. } => starts_at.date_naive() > next_week_end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_overdue() {
        let now = Utc::now();
        let yesterday = now - Duration::days(1);

        assert!(!TaskSchedule::Unscheduled.is_overdue());

        // AllDay
        assert!(TaskSchedule::AllDay {
            date: yesterday.date_naive()
        }
        .is_overdue());
        assert!(!TaskSchedule::AllDay {
            date: now.date_naive()
        }
        .is_overdue());

        // At
        assert!(TaskSchedule::At {
            starts_at: now - Duration::seconds(60)
        }
        .is_overdue());
        assert!(!TaskSchedule::At {
            starts_at: now + Duration::seconds(60)
        }
        .is_overdue());

        // Span
        assert!(TaskSchedule::Span {
            starts_at: yesterday,
            duration: Duration::hours(12),
        }
        .is_overdue()); // Ended 12 hours ago

        assert!(!TaskSchedule::Span {
            starts_at: yesterday,
            duration: Duration::days(2),
        }
        .is_overdue()); // Still ongoing
    }

    #[test]
    fn test_is_today() {
        let now = Utc::now();
        let tomorrow = now + Duration::days(1);

        assert!(!TaskSchedule::Unscheduled.is_today());

        // AllDay
        assert!(TaskSchedule::AllDay {
            date: now.date_naive()
        }
        .is_today());
        assert!(!TaskSchedule::AllDay {
            date: tomorrow.date_naive()
        }
        .is_today());

        // At
        assert!(TaskSchedule::At { starts_at: now }.is_today());
        assert!(!TaskSchedule::At {
            starts_at: tomorrow
        }
        .is_today());

        // Span
        assert!(TaskSchedule::Span {
            starts_at: now,
            duration: Duration::hours(1), // Assuming it doesn't cross midnight
        }
        .is_today());
    }

    #[test]
    fn test_is_next_week() {
        let now = Utc::now();
        let tomorrow = now + Duration::days(1);
        let in_five_days = now + Duration::days(5);
        let in_eight_days = now + Duration::days(8);

        assert!(!TaskSchedule::Unscheduled.is_next_week());

        // Should exclude today
        assert!(!TaskSchedule::AllDay {
            date: now.date_naive()
        }
        .is_next_week());

        // Should include next 7 days
        assert!(TaskSchedule::AllDay {
            date: tomorrow.date_naive()
        }
        .is_next_week());
        assert!(TaskSchedule::AllDay {
            date: in_five_days.date_naive()
        }
        .is_next_week());

        // Should exclude days after 7 days
        assert!(!TaskSchedule::AllDay {
            date: in_eight_days.date_naive()
        }
        .is_next_week());

        // Span checking
        assert!(TaskSchedule::Span {
            starts_at: in_five_days,
            duration: Duration::hours(2),
        }
        .is_next_week());
    }

    #[test]
    fn test_is_incoming() {
        let now = Utc::now();
        let in_five_days = now + Duration::days(5);
        let in_eight_days = now + Duration::days(8);

        assert!(!TaskSchedule::Unscheduled.is_incoming());

        // Excludes today and next week
        assert!(!TaskSchedule::AllDay {
            date: now.date_naive()
        }
        .is_incoming());
        assert!(!TaskSchedule::AllDay {
            date: in_five_days.date_naive()
        }
        .is_incoming());

        // Includes everything strictly after 7 days
        assert!(TaskSchedule::AllDay {
            date: in_eight_days.date_naive()
        }
        .is_incoming());

        assert!(TaskSchedule::At {
            starts_at: in_eight_days
        }
        .is_incoming());

        assert!(TaskSchedule::Span {
            starts_at: in_eight_days,
            duration: Duration::days(1),
        }
        .is_incoming());
    }
}
