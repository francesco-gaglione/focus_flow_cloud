use chrono::NaiveTime;
use thiserror::Error;

use crate::{
    entities::tasks::task::Task,
    value_objects::stats::peak_window::{
        PeakWindow, PeakWindowError, PwTimeRange, PwTimeRangeError,
    },
};

#[derive(Debug, Error, PartialEq)]
pub enum PeakWindowServiceError {
    #[error("time range overlaps with existing range")]
    TimeRangeOverlap,

    #[error("time range count exceeds limit")]
    TimeRangeCountExceedsLimit,

    #[error("invalid time range")]
    InvalidTimeRange,
}

impl From<PeakWindowError> for PeakWindowServiceError {
    fn from(value: PeakWindowError) -> Self {
        match value {
            PeakWindowError::TimeRangeOverlap => Self::TimeRangeOverlap,
            PeakWindowError::TimeRangeCountExceedsLimit => Self::TimeRangeCountExceedsLimit,
        }
    }
}

impl From<PwTimeRangeError> for PeakWindowServiceError {
    fn from(value: PwTimeRangeError) -> Self {
        match value {
            PwTimeRangeError::InvalidTimeRange => Self::InvalidTimeRange,
        }
    }
}

pub struct PeakWindowService {}

impl PeakWindowService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate(&self, tasks: &[Task]) -> Result<PeakWindow, PeakWindowServiceError> {
        let mut peak_window = PeakWindow::new();

        for (start, end) in [
            (
                NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
            ),
        ] {
            let tasks_in_range = tasks
                .iter()
                .filter(|t| {
                    t.completed_at().is_some()
                        && t.completed_at().unwrap().time() >= start
                        && t.completed_at().unwrap().time() <= end
                })
                .collect::<Vec<_>>();

            peak_window.add_time_range_count(PwTimeRange::new(
                start,
                end,
                tasks_in_range.len(),
            )?)?;
        }

        Ok(peak_window)
    }
}

#[cfg(test)]
mod tests_calculate_peak_window {
    use chrono::{NaiveDate, NaiveTime};
    use uuid::Uuid;

    use crate::entities::tasks::{task::Task, task_schedule::TaskSchedule};

    use super::*;

    fn t(h: u32, m: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, 0).unwrap()
    }

    fn completed_at(time: NaiveTime) -> Task {
        let dt = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_time(time)
            .and_utc();
        Task::reconstitute(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "task".to_string(),
            None,
            None,
            None,
            vec![],
            TaskSchedule::Unscheduled,
            Some(dt),
            vec![],
        )
    }

    fn not_completed() -> Task {
        Task::reconstitute(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "task".to_string(),
            None,
            None,
            None,
            vec![],
            TaskSchedule::Unscheduled,
            None,
            vec![],
        )
    }

    fn svc() -> PeakWindowService {
        PeakWindowService::new()
    }

    #[test]
    fn test_empty_tasks_produces_9_zero_ranges() {
        let result = svc().calculate(&[]).unwrap();
        assert_eq!(result.time_ranges().len(), 9);
        assert!(result.time_ranges().iter().all(|r| r.count() == 0));
    }

    #[test]
    fn test_uncompleted_tasks_not_counted() {
        let tasks = vec![not_completed(), not_completed()];
        let result = svc().calculate(&tasks).unwrap();
        assert!(result.time_ranges().iter().all(|r| r.count() == 0));
    }

    #[test]
    fn test_task_in_first_window() {
        let tasks = vec![completed_at(t(7, 0))];
        let ranges = svc().calculate(&tasks).unwrap();
        let ranges = ranges.time_ranges();
        assert_eq!(ranges[0].count(), 1);
        assert!(ranges[1..].iter().all(|r| r.count() == 0));
    }

    #[test]
    fn test_task_in_last_window() {
        let tasks = vec![completed_at(t(23, 0))];
        let ranges = svc().calculate(&tasks).unwrap();
        let ranges = ranges.time_ranges();
        assert_eq!(ranges[8].count(), 1);
        assert!(ranges[..8].iter().all(|r| r.count() == 0));
    }

    #[test]
    fn test_multiple_tasks_same_window() {
        let tasks = vec![
            completed_at(t(9, 0)),
            completed_at(t(9, 30)),
            completed_at(t(9, 59)),
        ];
        let ranges = svc().calculate(&tasks).unwrap();
        let ranges = ranges.time_ranges();
        assert_eq!(ranges[1].count(), 3); // 8:00-10:00
    }

    #[test]
    fn test_tasks_spread_across_windows() {
        let tasks = vec![
            completed_at(t(6, 30)),  // 6-8
            completed_at(t(9, 0)),   // 8-10
            completed_at(t(11, 0)),  // 10-12
            completed_at(t(13, 0)),  // 12-14
            completed_at(t(15, 0)),  // 14-16
            completed_at(t(17, 0)),  // 16-18
            completed_at(t(19, 0)),  // 18-20
            completed_at(t(21, 0)),  // 20-22
            completed_at(t(23, 0)),  // 22-24
        ];
        let result = svc().calculate(&tasks).unwrap();
        let ranges = result.time_ranges();
        for r in ranges {
            assert_eq!(r.count(), 1);
        }
    }

    #[test]
    fn test_tasks_before_first_window_not_counted() {
        let tasks = vec![completed_at(t(5, 59))];
        let result = svc().calculate(&tasks).unwrap();
        assert!(result.time_ranges().iter().all(|r| r.count() == 0));
    }

    #[test]
    fn test_mixed_completed_and_not() {
        let tasks = vec![
            completed_at(t(10, 30)),
            not_completed(),
            completed_at(t(10, 45)),
            not_completed(),
        ];
        let ranges = svc().calculate(&tasks).unwrap();
        let ranges = ranges.time_ranges();
        assert_eq!(ranges[2].count(), 2); // 10:00-12:00
        assert!(ranges.iter().enumerate().filter(|(i, _)| *i != 2).all(|(_, r)| r.count() == 0));
    }
}
