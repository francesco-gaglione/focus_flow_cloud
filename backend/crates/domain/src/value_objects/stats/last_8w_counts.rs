use chrono::{Datelike, Days, NaiveDate, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Last8wCountsError {
    #[error("week is not within the last 8 weeks")]
    WeekNotWithinLast8Weeks,
}

pub struct WeekCounts {
    week_start: NaiveDate,
    count: usize,
}

impl WeekCounts {
    pub fn new(week_start: NaiveDate, count: usize) -> Self {
        Self { week_start, count }
    }

    pub fn week_start(&self) -> NaiveDate {
        self.week_start
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

pub struct Last8wCounts {
    counts: Vec<WeekCounts>,
}

fn current_week_start() -> NaiveDate {
    let today = Utc::now().date_naive();
    let days_from_monday = today.weekday().num_days_from_monday();
    today
        .checked_sub_days(Days::new(days_from_monday as u64))
        .unwrap_or(today)
}

impl Default for Last8wCounts {
    fn default() -> Self {
        Self::new()
    }
}

impl Last8wCounts {
    pub fn new() -> Self {
        Self { counts: Vec::new() }
    }

    pub fn push(&mut self, week_start: NaiveDate, count: usize) -> Result<(), Last8wCountsError> {
        let this_week = current_week_start();
        let eight_weeks_ago = this_week
            .checked_sub_days(Days::new(7 * 7))
            .unwrap_or(NaiveDate::MIN);

        if week_start < eight_weeks_ago || week_start > this_week {
            return Err(Last8wCountsError::WeekNotWithinLast8Weeks);
        }

        if let Some(index) = self.counts.iter().position(|w| w.week_start == week_start) {
            let existing = self.counts[index].count();
            self.counts[index] = WeekCounts::new(week_start, existing + count);
            return Ok(());
        }

        self.counts.push(WeekCounts::new(week_start, count));
        Ok(())
    }

    pub fn counts(&self) -> &[WeekCounts] {
        &self.counts
    }

    pub fn total(&self) -> usize {
        self.counts.iter().map(|w| w.count).sum()
    }

    pub fn max(&self) -> usize {
        self.counts.iter().map(|w| w.count).max().unwrap_or(0)
    }

    pub fn find_by_week(&self, week_start: NaiveDate) -> Option<&WeekCounts> {
        self.counts.iter().find(|w| w.week_start == week_start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Days;

    fn this_week() -> NaiveDate {
        current_week_start()
    }

    fn weeks_ago(n: u64) -> NaiveDate {
        this_week().checked_sub_days(Days::new(7 * n)).unwrap()
    }

    #[test]
    fn test_week_counts_getters() {
        let wc = WeekCounts::new(this_week(), 4);
        assert_eq!(wc.week_start(), this_week());
        assert_eq!(wc.count(), 4);
    }

    #[test]
    fn test_push_this_week() {
        let mut c = Last8wCounts::new();
        assert!(c.push(this_week(), 5).is_ok());
        assert_eq!(c.counts().len(), 1);
        assert_eq!(c.total(), 5);
    }

    #[test]
    fn test_push_7_weeks_ago() {
        let mut c = Last8wCounts::new();
        assert!(c.push(weeks_ago(7), 2).is_ok());
    }

    #[test]
    fn test_push_too_old_rejected() {
        let mut c = Last8wCounts::new();
        let too_old = weeks_ago(8);
        assert!(matches!(
            c.push(too_old, 1),
            Err(Last8wCountsError::WeekNotWithinLast8Weeks)
        ));
    }

    #[test]
    fn test_push_future_rejected() {
        let mut c = Last8wCounts::new();
        let future = this_week().checked_add_days(Days::new(7)).unwrap();
        assert!(matches!(
            c.push(future, 1),
            Err(Last8wCountsError::WeekNotWithinLast8Weeks)
        ));
    }

    #[test]
    fn test_push_duplicate_accumulates() {
        let mut c = Last8wCounts::new();
        c.push(this_week(), 3).unwrap();
        c.push(this_week(), 5).unwrap();
        assert_eq!(c.counts().len(), 1);
        assert_eq!(c.find_by_week(this_week()).unwrap().count(), 8);
    }

    #[test]
    fn test_total() {
        let mut c = Last8wCounts::new();
        c.push(this_week(), 3).unwrap();
        c.push(weeks_ago(1), 7).unwrap();
        assert_eq!(c.total(), 10);
    }

    #[test]
    fn test_total_empty() {
        assert_eq!(Last8wCounts::new().total(), 0);
    }

    #[test]
    fn test_max() {
        let mut c = Last8wCounts::new();
        c.push(this_week(), 2).unwrap();
        c.push(weeks_ago(1), 9).unwrap();
        c.push(weeks_ago(2), 4).unwrap();
        assert_eq!(c.max(), 9);
    }

    #[test]
    fn test_max_empty() {
        assert_eq!(Last8wCounts::new().max(), 0);
    }

    #[test]
    fn test_find_by_week_found() {
        let mut c = Last8wCounts::new();
        c.push(this_week(), 6).unwrap();
        assert_eq!(c.find_by_week(this_week()).unwrap().count(), 6);
    }

    #[test]
    fn test_find_by_week_not_found() {
        let c = Last8wCounts::new();
        assert!(c.find_by_week(this_week()).is_none());
    }
}
