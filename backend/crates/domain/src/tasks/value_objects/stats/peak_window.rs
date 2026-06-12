use chrono::NaiveTime;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum PwTimeRangeError {
    #[error("invalid time range: end must be after start")]
    InvalidTimeRange,
}

pub struct PwTimeRange {
    start: NaiveTime,
    end: NaiveTime,
    count: usize,
}

impl PwTimeRange {
    pub fn new(start: NaiveTime, end: NaiveTime, count: usize) -> Result<Self, PwTimeRangeError> {
        if end <= start {
            return Err(PwTimeRangeError::InvalidTimeRange);
        }
        Ok(Self { start, end, count })
    }

    pub fn start(&self) -> NaiveTime {
        self.start
    }

    pub fn end(&self) -> NaiveTime {
        self.end
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum PeakWindowError {
    #[error("time range overlaps with existing range")]
    TimeRangeOverlap,

    #[error("time range count exceeds limit")]
    TimeRangeCountExceedsLimit,
}

pub struct PeakWindow {
    time_ranges: Vec<PwTimeRange>,
}

impl Default for PeakWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl PeakWindow {
    pub fn new() -> Self {
        Self {
            time_ranges: Vec::new(),
        }
    }

    /// Adds a time range count to the peak window, ensuring the count does not exceed 24 ranges
    // Currently, the peak window supports a maximum of 24 time ranges
    // in future implementations the limit will be configurable based on the dynamic time range (e.g. 30 minutes time interval, 48 time ranges count)
    pub fn add_time_range_count(&mut self, time_range: PwTimeRange) -> Result<(), PeakWindowError> {
        if self.time_ranges.len() >= 24 {
            return Err(PeakWindowError::TimeRangeCountExceedsLimit);
        }

        let has_overlap = self
            .time_ranges
            .iter()
            .any(|r| time_range.start < r.end && r.start < time_range.end);

        if has_overlap {
            return Err(PeakWindowError::TimeRangeOverlap);
        }

        self.time_ranges.push(time_range);
        Ok(())
    }

    pub fn time_ranges(&self) -> &[PwTimeRange] {
        &self.time_ranges
    }
}

#[cfg(test)]
pub mod peak_window_tests {
    use chrono::NaiveTime;

    use crate::tasks::value_objects::stats::peak_window::{
        PeakWindow, PeakWindowError, PwTimeRange, PwTimeRangeError,
    };

    fn t(h: u32, m: u32, s: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, s).unwrap()
    }

    #[test]
    fn tr_test_creation() {
        let start = t(8, 0, 0);
        let end = t(9, 0, 0);
        let time_range = PwTimeRange::new(start, end, 5);
        assert!(time_range.is_ok());
        let time_range = time_range.unwrap();
        assert_eq!(time_range.count(), 5);
        assert_eq!(time_range.start(), start);
        assert_eq!(time_range.end(), end);
    }

    #[test]
    fn tr_invalid_time_range() {
        let time_range = PwTimeRange::new(t(9, 0, 0), t(8, 0, 0), 5);
        assert!(time_range.is_err());
        assert_eq!(time_range.err(), Some(PwTimeRangeError::InvalidTimeRange));
    }

    #[test]
    fn pw_test_creation() {
        let mut peak_window = PeakWindow::new();
        let res =
            peak_window.add_time_range_count(PwTimeRange::new(t(8, 0, 0), t(9, 0, 0), 5).unwrap());
        assert!(res.is_ok());
        assert_eq!(peak_window.time_ranges().len(), 1);
    }

    #[test]
    fn pw_test_time_range_count_exceeds_limit() {
        let mut peak_window = PeakWindow::new();
        for i in 0u32..24 {
            let res = peak_window
                .add_time_range_count(PwTimeRange::new(t(0, 0, i), t(0, 0, i + 1), 7).unwrap());
            assert!(res.is_ok());
        }

        let res = peak_window
            .add_time_range_count(PwTimeRange::new(t(0, 0, 24), t(0, 0, 25), 7).unwrap());
        assert!(res.is_err());
        assert_eq!(res.err(), Some(PeakWindowError::TimeRangeCountExceedsLimit));
    }

    #[test]
    fn pw_test_time_range_overlap() {
        let mut peak_window = PeakWindow::new();

        let _ =
            peak_window.add_time_range_count(PwTimeRange::new(t(8, 0, 0), t(9, 0, 0), 7).unwrap());
        let res =
            peak_window.add_time_range_count(PwTimeRange::new(t(9, 0, 0), t(10, 0, 0), 7).unwrap());
        assert!(res.is_ok());

        let _ =
            peak_window.add_time_range_count(PwTimeRange::new(t(8, 0, 0), t(9, 0, 1), 7).unwrap());
        let res =
            peak_window.add_time_range_count(PwTimeRange::new(t(9, 0, 0), t(10, 0, 0), 7).unwrap());
        assert!(res.is_err());
        assert_eq!(res.err(), Some(PeakWindowError::TimeRangeOverlap));
    }
}
